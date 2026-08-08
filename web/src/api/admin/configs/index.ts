import type { AdminConfig, Statistics } from "@/models/config";
import { api } from "@/utils/query";

export async function getConfigs() {
  return api.get("admin/configs").json<{ config: AdminConfig }>();
}

export async function updateConfig(request: AdminConfig) {
  return api
    .put("admin/configs", { json: request })
    .json<{ config: AdminConfig }>();
}

export async function getStatistics() {
  return api.get("admin/configs/statistics").json<{ statistics: Statistics }>();
}
