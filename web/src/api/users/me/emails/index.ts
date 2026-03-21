import type { Email } from "@/models/email";
import { api } from "@/utils/query";

export async function getEmails() {
  return api.get(`users/me/emails`).json<{ items: Email[]; total: number }>();
}

export interface AddEmailRequest {
  email: string;
}

export async function addEmail(request: AddEmailRequest) {
  return api
    .post(`users/me/emails`, { json: request })
    .json<{ email: Email }>();
}

export interface DeleteEmailRequest {
  email: string;
}

export async function deleteEmail(request: DeleteEmailRequest) {
  return api
    .delete(`users/me/emails/${request.email}`, { json: request })
    .json<{ email: Email }>();
}

export interface VerifyEmailRequest {
  email: string;
  code: string;
}

export async function verifyEmail(request: VerifyEmailRequest) {
  return api
    .post(`users/me/emails/${request.email}/verify`, { json: request })
    .json<{ email: Email }>();
}

export interface SendVerifyEmailRequest {
  email: string;
}

export async function sendVerifyEmail(request: SendVerifyEmailRequest) {
  return api
    .post(`users/me/emails/${request.email}/verify/send`)
    .json<Record<string, never>>();
}
