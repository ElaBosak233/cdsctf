import { StatusCodes } from "http-status-codes";
import ky, { HTTPError, TimeoutError } from "ky";
import { toast } from "sonner";
import { useAuthStore } from "@/storages/auth";
import type { ErrorResponse } from "@/types";

const api = ky.extend({
  prefixUrl: "/api",
  timeout: 5000,
  hooks: {
    beforeError: [
      async (error) => {
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
      async (error) => {
        if (!(error instanceof TimeoutError)) return error;

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

export { api, parseErrorResponse, toSearchParams };
