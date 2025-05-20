import { WebResponse } from "@/types";
import { api } from "@/utils/ky";

export async function deleteUserAvatar() {
  return api.delete(`users/profile/avatar`).json<WebResponse<never>>();
}
