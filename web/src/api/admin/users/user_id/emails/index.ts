import type { Email } from "@/models/email";
import { api } from "@/utils/query";

export interface GetEmailsRequest {
  user_id: number;
}

export async function getEmails(request: GetEmailsRequest) {
  return api
    .get(`admin/users/${request.user_id}/emails`)
    .json<{ items: Email[]; total: number }>();
}

export interface AddEmailRequest {
  user_id: number;
  email: string;
  verified?: boolean;
}

export async function addEmail(request: AddEmailRequest) {
  return api
    .post(`admin/users/${request.user_id}/emails`, {
      json: request,
    })
    .json<{ email: Email }>();
}

export interface UpdateEmailRequest {
  user_id: number;
  email: string;
  verified: boolean;
}

export async function updateEmail(request: UpdateEmailRequest) {
  return api
    .put(
      `admin/users/${request.user_id}/emails/${encodeURIComponent(request.email)}`,
      {
        json: request,
      }
    )
    .json<{ email: Email }>();
}

export interface DeleteEmailRequest {
  user_id: number;
  email: string;
}

export async function deleteEmail(request: DeleteEmailRequest) {
  return api
    .delete(
      `admin/users/${request.user_id}/emails/${encodeURIComponent(request.email)}`
    )
    .json<Record<string, never>>();
}
