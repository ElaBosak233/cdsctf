import { Config, Version } from "@/models/config";
import { WebResponse } from "@/types";
import { api } from "@/utils/ky";

export async function getConfigs() {
  return api.get("configs").json<WebResponse<Config>>();
}

export async function getVersion() {
  return api.get("configs/version").json<WebResponse<Version>>();
}
