import type { Config, Statistics } from "@/models/config";
import { api } from "@/utils/query";

export async function getConfigs() {
  return api.get("admin/configs").json<{ config: Config }>();
}

export async function updateConfig(request: Config) {
  return api
    .put("admin/configs", { json: request })
    .json<{ config: Config }>();
}

export async function getStatistics() {
  return api.get("admin/configs/statistics").json<{ statistics: Statistics }>();
}
