import type { UserAccountView } from "@/models/user";
import { api } from "@/utils/query";

export type GetUserRequest = {
  id: number;
};

export async function getUser(request: GetUserRequest) {
  return api.get(`management/users/${request.id}`).json<{ user: UserAccountView }>();
}

export type DeleteUserRequest = {
  id: number;
};

export async function deleteUser(request: DeleteUserRequest) {
  return api
    .delete(`management/users/${request.id}`, { json: request })
    .json<Record<string, never>>();
}

export type UpdateUserRequest = {
  id: number;
  username?: string;
  name?: string;
  email?: string;
  banned?: boolean;
  permissions?: string[];
  password?: string;
  verified?: boolean;
  description?: string | null;
};

export async function updateUser(request: UpdateUserRequest) {
  return api
    .patch(`management/users/${request.id}`, { json: request })
    .json<{ user: UserAccountView }>();
}
