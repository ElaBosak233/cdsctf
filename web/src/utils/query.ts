import { StatusCodes } from "http-status-codes";
import ky, { HTTPError, TimeoutError } from "ky";
import { toast } from "sonner";
import { clearAuthenticatedUser } from "@/storages/auth";
import { decodeApiJson, type ErrorResponse } from "@/types";
import {
  API_ERROR_FALLBACK_KEY,
  API_ERROR_I18N_KEYS,
} from "@/utils/api-errors";
import i18n from "@/utils/i18n";

type PendingEntry = {
  controller: AbortController;
  resolve: (response: Response) => void;
  reject: (reason: unknown) => void;
  responsePromise: Promise<Response>;
};

const pendingRequests = new Map<string, PendingEntry>();
const handledErrors = new WeakSet<object>();

function markApiErrorHandled(error: unknown) {
  if (typeof error === "object" && error !== null) handledErrors.add(error);
}

function isApiErrorHandled(error: unknown) {
  return (
    typeof error === "object" && error !== null && handledErrors.has(error)
  );
}

function isApiError(error: unknown) {
  return (
    error instanceof HTTPError ||
    error instanceof TimeoutError ||
    (typeof error === "object" &&
      error !== null &&
      ("code" in error || "details" in error))
  );
}

export async function notifyApiError(
  error: unknown,
  options: { title?: string; id?: string } = {}
) {
  if (isApiErrorHandled(error)) return;
  markApiErrorHandled(error);

  if (error instanceof TimeoutError) {
    toast.error(options.title ?? i18n.t("common:errors.timeout"), {
      id: options.id ?? "timeout",
    });
    return;
  }

  const message = await getApiErrorMessage(error);
  toast.error(options.title ?? i18n.t("common:errors.default"), {
    id: options.id,
    description: message || undefined,
  });
}

export function onUnhandledApiError(event: PromiseRejectionEvent) {
  if (!isApiError(event.reason) || isApiErrorHandled(event.reason)) return;

  event.preventDefault();
  void notifyApiError(event.reason);
}

async function getApiErrorMessage(error: unknown): Promise<string> {
  if (error instanceof HTTPError) {
    try {
      const body = await parseErrorResponse(error);
      return formatApiErrorMessage(body);
    } catch {
      return "";
    }
  }

  if (
    typeof error === "object" &&
    error !== null &&
    ("code" in error || "details" in error)
  ) {
    return formatApiErrorMessage(error);
  }

  return typeof error === "string" ? error : "";
}

const api = ky.extend({
  prefix: "/api",
  timeout: 5000,
  parseJson: (text) => decodeApiJson(JSON.parse(text)),
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
          return existing.responsePromise.then((response) => response.clone());
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
          markApiErrorHandled(error);
          clearAuthenticatedUser();

          if (!error.request.headers.get("Ignore-Unauthorized")) {
            toast.error(i18n.t("account:guard.login_required"), {
              id: "please-login-first",
            });
          }
        }

        if (error.response.status === StatusCodes.BAD_GATEWAY) {
          markApiErrorHandled(error);
          toast.error(i18n.t("common:errors.service_unavailable"), {
            id: "502-backend-offline",
            description: i18n.t(
              "common:errors.service_unavailable_description"
            ),
          });
        }

        return error;
      },
      async ({ error }) => {
        if (!(error instanceof TimeoutError)) return error as unknown as Error;

        markApiErrorHandled(error);
        toast.error(i18n.t("common:errors.timeout"), {
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
  try {
    const payload: unknown = await error.response.clone().json();
    if (typeof payload === "object" && payload !== null && "code" in payload) {
      return payload as ErrorResponse;
    }
    return { code: "request_failed", details: payload };
  } catch {
    return { code: "request_failed" };
  }
}

/** Gets the stable error code from an API error envelope. */
export function getApiErrorCode(payload: unknown): string | undefined {
  if (typeof payload !== "object" || payload === null) return undefined;

  const record = payload as Record<string, unknown>;
  if (typeof record.code === "string" && record.code.trim()) {
    return record.code;
  }
  return undefined;
}

function translateApiErrorCode(code: string): string | undefined {
  const key = API_ERROR_I18N_KEYS[code];
  if (!key) return undefined;
  return i18n.exists(key) ? i18n.t(key) : undefined;
}

/** Turns an API error envelope into a translated, user-facing message. */
export function formatApiErrorMessage(payload: unknown): string {
  const code = getApiErrorCode(payload);
  if (code) {
    const translated = translateApiErrorCode(code);
    if (translated) return translated;
  }
  return i18n.t(API_ERROR_FALLBACK_KEY);
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
