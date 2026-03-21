import { api } from "@/utils/query";

export async function deleteLogo() {
  return api.delete("admin/configs/logo").json<Record<string, never>>();
}
