import { t } from "i18next";
import ky from "ky";
import { toast } from "sonner";
import globalRouter from "./global-router";

import { useAuthStore } from "@/storages/auth";

export const api = ky.extend({
  prefixUrl: "/api",
  timeout: 5000,
  hooks: {
    afterResponse: [
      (_request, _options, response) => {
        if (response.status === 401) {
          globalRouter?.navigate?.("/account/login");
          toast.warning(t("account:login.please"), {
            id: "please-login",
            description: "登录后才能继续操作",
          });
          useAuthStore?.getState()?.clear();
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

export function toSearchParams(
  obj: Record<string, any>
): Record<string, string | number | boolean> {
  return Object.fromEntries(
    Object.entries(obj).filter(([_, v]) => v !== undefined)
  ) as Record<string, string | number | boolean>;
}
