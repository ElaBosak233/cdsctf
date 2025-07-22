import type { WebResponse } from "@/types";
import { api } from "@/utils/query";

export async function deleteLogo() {
  return api.delete("admin/configs/logo").json<WebResponse<never>>();
}
