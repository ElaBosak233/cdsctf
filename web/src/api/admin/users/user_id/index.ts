import type { Group, User } from "@/models/user";
import { api } from "@/utils/query";

export type GetUserRequest = {
  id: number;
};

export async function getUser(request: GetUserRequest) {
  return api.get(`admin/users/${request.id}`).json<{ user: User }>();
}

export type DeleteUserRequest = {
  id: number;
};

export async function deleteUser(request: DeleteUserRequest) {
  return api
    .delete(`admin/users/${request.id}`, { json: request })
    .json<Record<string, never>>();
}

export type UpdateUserRequest = {
  id: number;
  username?: string;
  name?: string;
  email?: string;
  group?: Group;
  password?: string;
  verified?: boolean;
  description?: string | null;
};

export async function updateUser(request: UpdateUserRequest) {
  return api
    .put(`admin/users/${request.id}`, { json: request })
    .json<{ user: User }>();
}
