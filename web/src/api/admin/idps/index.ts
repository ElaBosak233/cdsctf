import type { Idp } from "@/models/idp";
import { api } from "@/utils/query";

export interface IdpRequest {
  name?: string;
  enabled?: boolean;
  portal?: string | null;
  script?: string;
}

export interface DiagnosticMarker {
  start_line: number;
  start_column: number;
  end_line: number;
  end_column: number;
  kind: "error" | "warning";
  message: string;
}

export async function getAdminIdps() {
  return api.get("admin/idps").json<{ idps: Idp[] }>();
}

export async function createAdminIdp(request: IdpRequest) {
  return api.post("admin/idps", { json: request }).json<{ idp: Idp }>();
}

export async function getAdminIdp(idpId: number) {
  return api.get(`admin/idps/${idpId}`).json<{ idp: Idp }>();
}

export async function updateAdminIdp(idpId: number, request: IdpRequest) {
  return api.put(`admin/idps/${idpId}`, { json: request }).json<{ idp: Idp }>();
}

export async function deleteAdminIdp(idpId: number) {
  return api.delete(`admin/idps/${idpId}`).json<Record<string, never>>();
}

export async function deleteAdminIdpAvatar(idpId: number) {
  return api.delete(`admin/idps/${idpId}/avatar`).json<Record<string, never>>();
}

export async function lintIdpScript(script: string) {
  return api
    .post("admin/idps/lint", { json: { script } })
    .json<{ markers: DiagnosticMarker[] }>();
}
