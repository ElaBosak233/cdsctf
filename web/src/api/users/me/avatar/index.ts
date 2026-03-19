import type { WebResponse } from "@/types";
import { api } from "@/utils/query";

export async function deleteUserAvatar() {
  return api.delete(`users/me/avatar`).json<WebResponse<never>>();
}
