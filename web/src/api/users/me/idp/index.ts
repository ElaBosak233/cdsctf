import type { UserIdpSummary } from "@/models/idp";
import { api } from "@/utils/query";

export async function getMyIdps() {
  return api
    .get("users/me/idps")
    .json<{ idps: UserIdpSummary[] }>();
}

export async function unbindMyIdp(userIdpId: number) {
  return api.delete(`users/me/idps/${userIdpId}`).json<Record<string, never>>();
}
