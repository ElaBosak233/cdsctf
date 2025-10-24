import type { Email } from "@/models/email";
import type { WebResponse } from "@/types";
import { api } from "@/utils/query";

export async function getEmails() {
  return api.get(`users/profile/emails`).json<WebResponse<Array<Email>>>();
}

export interface AddEmailRequest {
  email: string;
}

export async function addEmail(request: AddEmailRequest) {
  return api
    .post(`users/profile/emails`, { json: request })
    .json<WebResponse<Email>>();
}

export interface DeleteEmailRequest {
  email: string;
}

export async function deleteEmail(request: DeleteEmailRequest) {
  return api
    .delete(`users/profile/emails/${request.email}`, { json: request })
    .json<WebResponse<unknown>>();
}

export interface VerifyEmailRequest {
  email: string;
  code: string;
}

export async function verifyEmail(request: VerifyEmailRequest) {
  return api
    .post(`users/profile/emails/${request.email}/verify`, { json: request })
    .json<WebResponse<Email>>();
}

export interface SendVerifyEmailRequest {
  email: string;
}

export async function sendVerifyEmail(request: SendVerifyEmailRequest) {
  return api
    .post(`users/profile/${request.email}/verify/send`)
    .json<WebResponse<never>>();
}
