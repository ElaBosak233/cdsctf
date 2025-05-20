import { Group, User } from "@/models/user";
import { WebResponse } from "@/types";
import { api, toSearchParams } from "@/utils/ky";

export interface GetUserRequest {
  id?: number;
  name?: string;
  username?: string;
  email?: string;
  group?: Group;
  page?: number;
  size?: number;
  sorts?: string;
}

export async function getUsers(request: GetUserRequest) {
  return api
    .get("admin/users", {
      searchParams: toSearchParams(request),
    })
    .json<WebResponse<Array<User>>>();
}

export interface CreateUserRequest {
  name?: string;
  username?: string;
  email?: string;
  group?: Group;
  password?: string;
}

export async function createUser(request: CreateUserRequest) {
  return api
    .post("admin/users", {
      json: request,
    })
    .json<WebResponse<User>>();
}
