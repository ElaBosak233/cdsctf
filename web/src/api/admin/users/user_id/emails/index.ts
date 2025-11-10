import type { Email } from "@/models/email";
import type { WebResponse } from "@/types";
import { api } from "@/utils/query";

export interface AdminUserScope {
  user_id: number;
}

export async function getAdminUserEmails(request: AdminUserScope) {
  return api
    .get(`admin/users/${request.user_id}/emails`)
    .json<WebResponse<Array<Email>>>();
}

export interface AdminAddEmailRequest extends AdminUserScope {
  email: string;
  is_verified?: boolean;
}

export async function addAdminUserEmail(request: AdminAddEmailRequest) {
  const { user_id, email, is_verified } = request;

  return api
    .post(`admin/users/${user_id}/emails`, {
      json: {
        email,
        is_verified,
      },
    })
    .json<WebResponse<Email>>();
}

export interface AdminUpdateEmailRequest extends AdminUserScope {
  email: string;
  is_verified: boolean;
}

export async function updateAdminUserEmail(request: AdminUpdateEmailRequest) {
  const { user_id, email, is_verified } = request;

  return api
    .put(`admin/users/${user_id}/emails/${encodeURIComponent(email)}`, {
      json: {
        is_verified,
      },
    })
    .json<WebResponse<Email>>();
}

export interface AdminDeleteEmailRequest extends AdminUserScope {
  email: string;
}

export async function deleteAdminUserEmail(request: AdminDeleteEmailRequest) {
  const { user_id, email } = request;

  return api
    .delete(`admin/users/${user_id}/emails/${encodeURIComponent(email)}`)
    .json<WebResponse<never>>();
}

export interface AdminSendVerifyEmailRequest extends AdminUserScope {
  email: string;
}

export async function sendAdminVerifyEmail(
  request: AdminSendVerifyEmailRequest
) {
  const { user_id, email } = request;

  return api
    .post(
      `admin/users/${user_id}/emails/${encodeURIComponent(email)}/verify/send`
    )
    .json<WebResponse<never>>();
}
