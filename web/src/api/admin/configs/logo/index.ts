import { alova } from "@/utils/alova";
import { WebResponse } from "@/types";

export async function deleteLogo() {
    return alova.Delete<WebResponse<never>>("/admin/configs/logo");
}
