import { StatusCodes } from "http-status-codes";
import ky, { HTTPError, NetworkError, TimeoutError } from "ky";
import { toast } from "sonner";
import { clearAuthenticatedUser } from "@/storages/auth";
import { decodeApiJson, type ErrorResponse } from "@/types";
import {
  API_ERROR_FALLBACK_KEYS,
  API_ERROR_I18N_KEYS,
  type ApiErrorI18nKeys,
  NETWORK_ERROR_I18N_KEYS,
  SERVICE_UNAVAILABLE_I18N_KEYS,
  TIMEOUT_ERROR_I18N_KEYS,
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
    error instanceof NetworkError ||
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
    showApiErrorToast(TIMEOUT_ERROR_I18N_KEYS, options);
    return;
  }

  if (error instanceof NetworkError) {
    showApiErrorToast(NETWORK_ERROR_I18N_KEYS, options);
    return;
  }

  const message = await getApiErrorToast(error);
  showApiErrorToast(message, options);
}

export function onUnhandledApiError(event: PromiseRejectionEvent) {
  if (!isApiError(event.reason) || isApiErrorHandled(event.reason)) return;

  event.preventDefault();
  void notifyApiError(event.reason);
}

export type ApiErrorToast = {
  title: string;
  description?: string;
};

function translateMessage(keys: ApiErrorI18nKeys): ApiErrorToast {
  const title = i18n.t(keys.title, { defaultValue: "" });
  const description = keys.description
    ? i18n.t(keys.description, { defaultValue: "" })
    : undefined;

  return {
    title: typeof title === "string" && title !== keys.title ? title : "",
    description:
      typeof description === "string" && description !== keys.description
        ? description
        : undefined,
  };
}

function showApiErrorToast(
  message: ApiErrorToast,
  options: { title?: string; id?: string } = {}
) {
  const actionTitle = options.title?.trim();
  const title = actionTitle || message.title;
  const descriptionParts = actionTitle
    ? [message.title, message.description]
    : [message.description];
  const description = [
    ...new Set(
      descriptionParts.filter(
        (part): part is string => Boolean(part) && part !== title
      )
    ),
  ].join(" — ");

  toast.error(title || translateMessage(API_ERROR_FALLBACK_KEYS).title, {
    id: options.id,
    description: description || undefined,
  });
}

async function getApiErrorToast(error: unknown): Promise<ApiErrorToast> {
  if (error instanceof HTTPError) {
    try {
      const body = await parseErrorResponse(error);
      return formatApiError(body);
    } catch {
      return translateMessage(API_ERROR_FALLBACK_KEYS);
    }
  }

  if (
    typeof error === "object" &&
    error !== null &&
    ("code" in error || "details" in error)
  ) {
    const record = error as Record<string, unknown>;
    if (typeof record.code === "string" && record.code.trim()) {
      return formatApiError(error);
    }
    if (typeof record.status === "number") {
      return formatApiError({
        code: getHttpStatusErrorCode(record.status),
        details: record.details,
      });
    }
    return formatApiError(error);
  }

  return typeof error === "string"
    ? { title: error }
    : translateMessage(API_ERROR_FALLBACK_KEYS);
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
            toast.error(i18n.t("account:guard.login_required.title"), {
              id: "please-login-first",
              description: i18n.t("account:guard.login_required.description"),
            });
          }
        }

        if (error.response.status === StatusCodes.BAD_GATEWAY) {
          markApiErrorHandled(error);
          showApiErrorToast(SERVICE_UNAVAILABLE_I18N_KEYS, {
            id: "502-backend-offline",
          });
        }

        return error;
      },
      async ({ error }) => {
        if (!(error instanceof TimeoutError)) return error as unknown as Error;

        markApiErrorHandled(error);
        showApiErrorToast(TIMEOUT_ERROR_I18N_KEYS, { id: "timeout" });

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

function isErrorResponse(payload: unknown): payload is ErrorResponse {
  if (typeof payload !== "object" || payload === null) return false;
  return typeof (payload as Record<string, unknown>).code === "string";
}

function getHttpStatusErrorCode(status: number): string {
  switch (status) {
    case StatusCodes.BAD_REQUEST:
      return "bad_request";
    case StatusCodes.UNAUTHORIZED:
      return "unauthorized";
    case StatusCodes.FORBIDDEN:
      return "forbidden";
    case StatusCodes.NOT_FOUND:
      return "not_found";
    case StatusCodes.CONFLICT:
      return "conflict";
    case StatusCodes.LOCKED:
      return "locked";
    case StatusCodes.TOO_MANY_REQUESTS:
      return "too_many_requests";
    case StatusCodes.UNPROCESSABLE_ENTITY:
      return "unprocessable_entity";
    default:
      return status >= 500 ? "internal_server_error" : "bad_request";
  }
}

/** Parses the JSON error payload already consumed by Ky into `HTTPError.data`. */
async function parseErrorResponse(error: HTTPError): Promise<ErrorResponse> {
  if (isErrorResponse(error.data)) return error.data;

  // Ky 2 consumes the response body before throwing HTTPError. This fallback
  // supports custom fetch implementations where data was not populated.
  if (error.data === undefined && !error.response.bodyUsed) {
    try {
      const payload: unknown = await error.response.clone().json();
      if (isErrorResponse(payload)) return payload;
      if (payload !== undefined) {
        return {
          code: getHttpStatusErrorCode(error.response.status),
          details: payload,
        };
      }
    } catch {
      // Fall through to a status-derived code below.
    }
  }

  return {
    code: getHttpStatusErrorCode(error.response.status),
    details: error.data,
  };
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

function translateApiErrorCode(code: string): ApiErrorToast | undefined {
  const keys = API_ERROR_I18N_KEYS[code];
  if (!keys) return undefined;

  const message = translateMessage(keys);
  return message.title ? message : undefined;
}

/** Turns an API error envelope into a translated, two-level toast message. */
export function formatApiError(payload: unknown): ApiErrorToast {
  const code = getApiErrorCode(payload);
  if (code) {
    const translated = translateApiErrorCode(code);
    if (translated) return translated;
  }
  return translateMessage(API_ERROR_FALLBACK_KEYS);
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
