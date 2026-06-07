import { api } from "@/utils/query";

export type UserForgetRequest = {
  email?: string;
  code?: string;
  password?: string;
}

export async function forget(request: UserForgetRequest) {
  return api
    .post(`users/forget`, { json: request })
    .json<Record<string, never>>();
}

export type UserSendForgetEmailRequest = {
  email?: string;
}

export async function sendForgetEmail(request: UserSendForgetEmailRequest) {
  return api
    .post(`users/forget/send`, { json: request })
    .json<Record<string, never>>();
}
