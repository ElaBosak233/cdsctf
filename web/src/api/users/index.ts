import { User } from "@/models/user";
import { WebResponse } from "@/types";
import { api } from "@/utils/query";

export interface UserLoginRequest {
  account: string;
  password: string;
  captcha?: {
    id?: string;
    content?: string;
  } | null;
}

export async function login(request: UserLoginRequest) {
  return api.post("users/login", { json: request }).json<WebResponse<User>>();
}

export async function logout() {
  return api.post("users/logout").json<WebResponse<never>>();
}

export interface UserRegisterRequest {
  username: string;
  name?: string;
  email: string;
  password: string;
  captcha?: {
    id?: string;
    content?: string;
  };
}

export async function register(request: UserRegisterRequest) {
  return api
    .post("users/register", { json: request })
    .json<WebResponse<User>>();
}
