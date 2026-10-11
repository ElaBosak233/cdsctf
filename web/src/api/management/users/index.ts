import type { Permission, UserAccountView } from "@/models/user";
import { api, toSearchParams } from "@/utils/query";

export type GetUsersRequest = {
  id?: number;
  name?: string;
  username?: string;
  email?: string;
  banned?: boolean;
  page?: number;
  size?: number;
  sorts?: string;
};

export async function getUsers(request: GetUsersRequest) {
  return api
    .get("management/users", {
      searchParams: toSearchParams(request),
    })
    .json<{ users: UserAccountView[]; total: number }>();
}

export type CreateUserRequest = {
  name?: string;
  username?: string;
  email?: string;
  banned?: boolean;
  permissions?: Permission[];
  password?: string;
};

export async function createUser(request: CreateUserRequest) {
  return api
    .post("management/users", {
      json: request,
    })
    .json<{ user: UserAccountView }>();
}
