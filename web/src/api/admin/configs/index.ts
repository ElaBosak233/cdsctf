import { Config } from "@/models/config";
import { WebResponse } from "@/types";
import { alova } from "@/utils/alova";

export async function getConfigs() {
  return alova.Get<WebResponse<Config>>("/admin/configs");
}

export async function updateConfig(request: Config) {
  return alova.Put<WebResponse<Config>>("/admin/configs", request);
}
