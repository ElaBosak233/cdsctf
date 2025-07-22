import type { Config, Statistics } from "@/models/config";
import type { WebResponse } from "@/types";
import { api } from "@/utils/query";

export async function getConfigs() {
  return api.get("admin/configs").json<WebResponse<Config>>();
}

export async function updateConfig(request: Config) {
  return api
    .put("admin/configs", { json: request })
    .json<WebResponse<Config>>();
}

export async function getStatistics() {
  return api.get("admin/configs/statistics").json<WebResponse<Statistics>>();
}
