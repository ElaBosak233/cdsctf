import { api } from "@/utils/query";

export async function deleteUserAvatar() {
  return api.delete(`users/me/avatar`).json<Record<string, never>>();
}
