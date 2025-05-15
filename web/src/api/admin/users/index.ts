import { Group, User } from "@/models/user";
import { WebResponse } from "@/types";
import { alova } from "@/utils/alova";

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
  return alova.Get<WebResponse<Array<User>>>("/admin/users", {
    params: request,
  });
}

export interface CreateUserRequest {
  name?: string;
  username?: string;
  email?: string;
  group?: Group;
  password?: string;
}

export async function createUser(request: CreateUserRequest) {
  return alova.Post<WebResponse<User>>("/admin/users", request);
}
