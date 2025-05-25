import { WebResponse } from "@/types";
import { api } from "@/utils/query";

export async function deleteUserAvatar() {
  return api.delete(`users/profile/avatar`).json<WebResponse<never>>();
}
