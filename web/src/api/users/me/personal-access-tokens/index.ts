import type { Timestamp } from "@/types";
import { api } from "@/utils/query";

export type PersonalAccessToken = {
  id: number;
  name: string;
  expires_at: Timestamp;
  last_used_at: Timestamp | null;
  revoked_at: Timestamp | null;
  created_at: Timestamp;
};

export type CreatePersonalAccessTokenRequest = {
  name: string;
  expires_at: string;
  captcha?: {
    id?: string;
    content?: string;
  } | null;
};

export async function getPersonalAccessTokens() {
  return api
    .get("users/me/personal-access-tokens")
    .json<{ personal_access_tokens: PersonalAccessToken[] }>();
}

export async function createPersonalAccessToken(
  request: CreatePersonalAccessTokenRequest
) {
  return api.post("users/me/personal-access-tokens", { json: request }).json<{
    token: string;
    personal_access_token: PersonalAccessToken;
  }>();
}

export async function revokePersonalAccessToken(id: number) {
  return api
    .delete(`users/me/personal-access-tokens/${id}`)
    .json<Record<string, never>>();
}
