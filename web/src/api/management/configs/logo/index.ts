import { api } from "@/utils/query";

export async function deleteLogo() {
  return api.delete("management/configs/logo").json<Record<string, never>>();
}
