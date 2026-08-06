import type { IdpSummary, UserIdpSummary } from "@/models/idp";
import type { UserAccountView } from "@/models/user";
import { api } from "@/utils/query";

export type IdpAuthRequest = {
  params?: Record<string, string>;
  captcha?: {
    id?: string;
    content?: string;
  } | null;
};

export async function getIdp(idpId: number) {
  return api.get(`idps/${idpId}`).json<{ idp: IdpSummary }>();
}

export async function getIdps() {
  return api.get("idps").json<{ idps: IdpSummary[] }>();
}

export async function loginWithIdp(idpId: number, request: IdpAuthRequest) {
  return api.post(`idps/${idpId}/login`, { json: request }).json<{
    user?: UserAccountView;
    registered: boolean;
    requires_registration?: boolean;
    pending_identity?: {
      token: string;
      idp_id: number;
      data: Record<string, string>;
    };
  }>();
}

export async function bindWithIdp(idpId: number, request: IdpAuthRequest) {
  return api
    .post(`idps/${idpId}/bind`, { json: request })
    .json<{ idp: UserIdpSummary }>();
}

export async function registerWithIdp(
  idpId: number,
  request: {
    token: string;
    username: string;
    name: string;
    email: string;
    password: string;
  }
) {
  return api
    .post(`idps/${idpId}/register`, { json: request })
    .json<{ user: UserAccountView }>();
}
