import { WebResponse } from "@/types";
import { api } from "@/utils/ky";

export async function deleteLogo() {
  return api.delete("admin/configs/logo").json<WebResponse<never>>();
}
