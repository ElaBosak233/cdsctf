import type { EmailView } from "@/models/email";
import i18n from "@/utils/i18n";
import { api, toSearchParams } from "@/utils/query";

export async function getEmails() {
  return api
    .get(`users/me/emails`)
    .json<{ emails: EmailView[]; total: number }>();
}

export type AddEmailRequest = {
  email: string;
  code?: string;
};

export async function addEmail(request: AddEmailRequest) {
  return api
    .post(`users/me/emails`, { json: request })
    .json<{ email: EmailView }>();
}

export type DeleteEmailRequest = {
  email: string;
};

export async function deleteEmail(request: DeleteEmailRequest) {
  return api
    .delete(`users/me/emails/${encodeURIComponent(request.email)}`)
    .json<Record<string, never>>();
}

export type VerifyEmailRequest = {
  email: string;
  code: string;
};

export async function verifyEmail(request: VerifyEmailRequest) {
  return api
    .post(`users/me/emails/${encodeURIComponent(request.email)}/verify`, {
      json: request,
    })
    .json<Record<string, never>>();
}

export type SendVerifyEmailRequest = {
  email: string;
};

export async function sendVerifyEmail(request: SendVerifyEmailRequest) {
  return api
    .post(`users/me/emails/${encodeURIComponent(request.email)}/verify/send`, {
      searchParams: toSearchParams({ locale: i18n.resolvedLanguage }),
    })
    .json<{ retry_after_seconds: number }>();
}

export async function sendBindEmail(request: { email: string }) {
  return api
    .post("users/me/emails/bind/send", {
      json: request,
      searchParams: toSearchParams({ locale: i18n.resolvedLanguage }),
    })
    .json<{ retry_after_seconds: number }>();
}
