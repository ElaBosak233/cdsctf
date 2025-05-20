import { Config } from "@/models/config";
import { WebResponse } from "@/types";
import { api } from "@/utils/ky";

export async function getConfigs() {
  return api.get("admin/configs").json<WebResponse<Config>>();
}

export async function updateConfig(request: Config) {
  return api
    .put("admin/configs", { json: request })
    .json<WebResponse<Config>>();
}
