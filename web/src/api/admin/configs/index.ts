import { alova } from "@/utils/alova";
import { WebResponse } from "@/types";
import { Config } from "@/models/config";

export async function getConfigs() {
    return alova.Get<WebResponse<Config>>("/admin/configs");
}

export async function updateConfig(request: Config) {
    return alova.Put<WebResponse<Config>>("/admin/configs", request);
}
