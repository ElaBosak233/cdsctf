import { Config, Version } from "@/models/config";
import { WebResponse } from "@/types";
import { alova } from "@/utils/alova";

export async function getConfigs() {
  return alova.Get<WebResponse<Config>>("/configs");
}

export async function getVersion() {
  return alova.Get<WebResponse<Version>>("/configs/version");
}
