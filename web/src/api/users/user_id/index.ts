import type { UserProfile } from "@/models/user";
import { api } from "@/utils/query";

export type GetUserRequest = {
  id: number;
};

export async function getUser(request: GetUserRequest) {
  return api.get(`users/${request.id}`).json<{ user: UserProfile }>();
}
