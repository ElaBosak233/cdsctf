import ky from "ky";
import { toast } from "sonner";

import { useAuthStore } from "@/storages/auth";

const api = ky.extend({
  prefixUrl: "/api",
  timeout: 5000,
  hooks: {
    afterResponse: [
      (_request, _options, response) => {
        if (response.status === 401) {
          useAuthStore?.getState()?.clear();
          toast.error("请先登录", {
            id: "please-login-first",
          });
          return Promise.reject(response);
        }

        if (response.status === 502) {
          toast.error("服务器离线", {
            id: "502-backend-offline",
            description: "服务器暂时无法处理请求",
          });
          return Promise.reject(response);
        }
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

export { api, toSearchParams };
