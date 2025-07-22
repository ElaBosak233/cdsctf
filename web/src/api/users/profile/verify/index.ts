import type { WebResponse } from "@/types";
import { api } from "@/utils/query";

export interface UserVerifyRequest {
  code?: string;
}

export async function verify(request: UserVerifyRequest) {
  return api
    .post(`users/profile/verify`, { json: request })
    .json<WebResponse<never>>();
}

export async function sendVerifyEmail() {
  return api.post(`users/profile/verify/send`).json<WebResponse<never>>();
}
