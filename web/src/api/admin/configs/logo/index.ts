import { WebResponse } from "@/types";
import { alova } from "@/utils/alova";

export async function deleteLogo() {
  return alova.Delete<WebResponse<never>>("/admin/configs/logo");
}
