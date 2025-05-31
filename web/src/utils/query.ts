import ky, { HTTPError, TimeoutError } from "ky";
import { toast } from "sonner";

import { useAuthStore } from "@/storages/auth";
import { WebResponse } from "@/types";
import { StatusCodes } from "http-status-codes";

const api = ky.extend({
  prefixUrl: "/api",
  timeout: 5000,
  hooks: {
    beforeError: [
      async (error) => {
        if (!(error instanceof HTTPError)) return error;
        const res = await parseErrorResponse(error);

        if (res.code === StatusCodes.UNAUTHORIZED) {
          useAuthStore?.getState()?.clear();

          if (!error.request.headers.get("Ignore-Unauthorized")) {
            toast.error("请先登录", {
              id: "please-login-first",
            });
          }
        }

        if (res.code === StatusCodes.BAD_GATEWAY) {
          toast.error("服务器离线", {
            id: "502-backend-offline",
            description: "服务器暂时无法处理请求",
          });
        }

        return error;
      },
      async (error) => {
        if (!(error instanceof TimeoutError)) return error;

        toast.error("请求超时", {
          id: "timeout",
        });

        return error;
      },
    ],
  },
});

function toSearchParams(
  obj: Record<string, any>
): Record<string, string | number | boolean> {
  return Object.fromEntries(
    Object.entries(obj).filter(([_, v]) => v !== undefined)
  ) as Record<string, string | number | boolean>;
}

async function parseErrorResponse(
  error: HTTPError
): Promise<WebResponse<unknown>> {
  return await error.response.clone().json();
}

export { api, parseErrorResponse, toSearchParams };
