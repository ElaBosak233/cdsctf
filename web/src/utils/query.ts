import { StatusCodes } from "http-status-codes";
import ky, { HTTPError, TimeoutError } from "ky";
import { toast } from "sonner";
import { useAuthStore } from "@/storages/auth";
import type { ErrorResponse } from "@/types";

type PendingEntry = {
  controller: AbortController;
  resolve: (response: Response) => void;
  reject: (reason: unknown) => void;
  responsePromise: Promise<Response>;
};

const pendingRequests = new Map<string, PendingEntry>();

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
          // A request is already in-flight — share its response instead of
          // making a duplicate call to the server.
          return existing.responsePromise;
        }

        let resolve: (response: Response) => void;
        let reject: (reason: unknown) => void;
        const responsePromise = new Promise<Response>((res, rej) => {
          resolve = res;
          reject = rej;
        });

        const controller = new AbortController();
        pendingRequests.set(key, {
          controller,
          resolve: resolve!,
          reject: reject!,
          responsePromise,
        });

        return new Request(request, { signal: controller.signal });
      },
    ],
    afterResponse: [
      ({ request, response }) => {
        if (!["GET", "HEAD"].includes(request.method)) return;
        const key = `${request.method}:${request.url}`;
        const pending = pendingRequests.get(key);
        if (pending) {
          pending.resolve(response.clone());
        }
        pendingRequests.delete(key);
      },
    ],
    beforeError: [
      async ({ request, error }) => {
        if (["GET", "HEAD"].includes(request.method)) {
          const key = `${request.method}:${request.url}`;
          const pending = pendingRequests.get(key);
          if (pending) {
            pending.reject(error);
          }
          pendingRequests.delete(key);
        }
        if (!(error instanceof HTTPError)) return error;

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

        return error;
      },
      async ({ error }) => {
        if (!(error instanceof TimeoutError)) return error as unknown as Error;

        toast.error("Request timed out", {
          id: "timeout",
        });

        return error;
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
