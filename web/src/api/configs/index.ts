import { alova } from "@/utils/alova";
import { WebResponse } from "@/types";
import { Config, Version } from "@/models/config";

export async function getConfigs() {
    return alova.Get<WebResponse<Config>>("/configs");
}

export async function getVersion() {
    return alova.Get<WebResponse<Version>>("/configs/version");
}
