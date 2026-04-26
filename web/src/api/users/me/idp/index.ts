import type { UserIdp } from "@/models/idp";
import { api } from "@/utils/query";

export async function getMyIdps() {
  return api.get("users/me/idp").json<{ idps: UserIdp[] }>();
}

export async function unbindMyIdp(userIdpId: number) {
  return api.delete(`users/me/idp/${userIdpId}`).json<Record<string, never>>();
}
