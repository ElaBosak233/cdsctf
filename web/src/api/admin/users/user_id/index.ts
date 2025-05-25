import { Group, User } from "@/models/user";
import { WebResponse } from "@/types";
import { api } from "@/utils/query";

export interface DeleteUserRequest {
  id: number;
}

export async function deleteUser(request: DeleteUserRequest) {
  return api
    .delete(`admin/users/${request.id}`, { json: request })
    .json<WebResponse<never>>();
}

export interface UpdateUserRequest {
  id: number;
  username?: string;
  name?: string;
  email?: string;
  group?: Group;
  password?: string;
  is_verified?: boolean;
  description?: string | null;
}

export async function updateUser(request: UpdateUserRequest) {
  return api
    .put(`admin/users/${request.id}`, { json: request })
    .json<WebResponse<User>>();
}
