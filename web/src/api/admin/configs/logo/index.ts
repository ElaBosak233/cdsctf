import { api } from "@/utils/query";

export async function deleteLogo() {
  return api.delete("configs/logo").json<Record<string, never>>();
}
