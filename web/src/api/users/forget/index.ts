import type { WebResponse } from "@/types";
import { api } from "@/utils/query";

export interface UserForgetRequest {
  email?: string;
  code?: string;
  password?: string;
}

export async function forget(request: UserForgetRequest) {
  return api.post(`users/forget`, { json: request }).json<WebResponse<never>>();
}

export interface UserSendForgetEmailRequest {
  email?: string;
}

export async function sendForgetEmail(request: UserSendForgetEmailRequest) {
  return api
    .post(`users/forget/send`, { json: request })
    .json<WebResponse<never>>();
}
