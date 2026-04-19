import { StatusCodes } from "http-status-codes";
import ky, { HTTPError, TimeoutError } from "ky";
import { toast } from "sonner";
import { useAuthStore } from "@/storages/auth";
import type { ErrorResponse } from "@/types";

const pendingRequests = new Map<string, AbortController>();

const api = ky.extend({
  prefix: "/api",
  timeout: 5000,
  hooks: {
    beforeRequest: [
      ({ request }) => {
        // only deduplicate non-mutating requests
        if (!["GET", "HEAD"].includes(request.method)) return;
        const key = `${request.method}:${request.url}`;
        const existing = pendingRequests.get(key);
        if (existing) {
          existing.abort();
        }
        const controller = new AbortController();
        pendingRequests.set(key, controller);
        return new Request(request, { signal: controller.signal });
      },
    ],
    afterResponse: [
      ({ request }) => {
        if (!["GET", "HEAD"].includes(request.method)) return;
        pendingRequests.delete(`${request.method}:${request.url}`);
      },
    ],
    beforeError: [
      async (error) => {
        const { request } = error;
        if (["GET", "HEAD"].includes(request.method)) {
          pendingRequests.delete(`${request.method}:${request.url}`);
        }
        if (!(error instanceof HTTPError)) return error as unknown as Error;

        if (error.response.status === StatusCodes.UNAUTHORIZED) {
          useAuthStore?.getState()?.clear();

          if (!error.request.headers.get("Ignore-Unauthorized")) {
            toast.error("Please sign in to continue", {
              id: "please-login-first",
            });
          }
        }

        if (error.response.status === StatusCodes.BAD_GATEWAY) {
          toast.error("Service unavailable", {
            id: "502-backend-offline",
            description: "The server could not complete the request.",
          });
        }

        return error as unknown as Error;
      },
      async (error) => {
        if (!(error instanceof TimeoutError)) return error as unknown as Error;

        toast.error("Request timed out", {
          id: "timeout",
        });

        return error as unknown as Error;
      },
    ],
  },
});

function toSearchParams<T extends object>(obj: T): URLSearchParams {
  const sp = new URLSearchParams();
  for (const [k, v] of Object.entries(obj as Record<string, unknown>)) {
    if (v !== undefined && v !== null) sp.append(k, String(v));
  }
  return sp;
}

/** Parses the JSON error payload from a failed ky request (`ErrorResponse`). */
async function parseErrorResponse(error: HTTPError): Promise<ErrorResponse> {
  return await error.response.clone().json();
}

/** Turns `msg` from [`ErrorResponse`] into a short string for toasts and form errors. */
export function formatApiMsg(msg: unknown): string {
  if (msg === undefined || msg === null) return "";
  if (typeof msg === "string") return msg;
  if (typeof msg === "number" || typeof msg === "boolean") return String(msg);
  try {
    return JSON.stringify(msg);
  } catch {
    return String(msg);
  }
}

/** Finite numeric id from a route param (`useParams`). Empty or non-numeric → `undefined`. */
export function parseRouteNumericId(
  raw: string | undefined
): number | undefined {
  if (raw == null || raw === "") return undefined;
  const n = Number(raw);
  return Number.isFinite(n) ? n : undefined;
}

export { api, parseErrorResponse, toSearchParams };
