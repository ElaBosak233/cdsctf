import type { PublicConfig, Version } from "@/models/config";
import { api } from "@/utils/query";

export async function getConfigs() {
  return api.get("configs").json<{ config: PublicConfig }>();
}

export async function getVersion() {
  return api.get("configs/version").json<Version>();
}
