import type { IdpView } from "@/models/idp";
import { api } from "@/utils/query";

export type IdpRequest = {
  name: string;
  enabled: boolean;
  registration_enabled: boolean;
  portal: string | null;
  script: string;
};

export type DiagnosticMarker = {
  start_line: number;
  start_column: number;
  end_line: number;
  end_column: number;
  kind: "error" | "warning";
  message: string;
};

export async function getAdminIdps() {
  return api.get("management/idps").json<{ idps: IdpView[] }>();
}

export async function createAdminIdp(request: IdpRequest) {
  return api
    .post("management/idps", { json: request })
    .json<{ idp: IdpView }>();
}

export async function getAdminIdp(idpId: number) {
  return api.get(`management/idps/${idpId}`).json<{ idp: IdpView }>();
}

export async function updateAdminIdp(idpId: number, request: IdpRequest) {
  return api
    .patch(`management/idps/${idpId}`, { json: request })
    .json<{ idp: IdpView }>();
}

export async function deleteAdminIdp(idpId: number) {
  return api.delete(`management/idps/${idpId}`).json<Record<string, never>>();
}

export async function deleteAdminIdpAvatar(idpId: number) {
  return api
    .delete(`management/idps/${idpId}/avatar`)
    .json<Record<string, never>>();
}

export async function lintIdpScript(script: string) {
  return api
    .post("management/idps/lint", { json: { script } })
    .json<{ markers: DiagnosticMarker[] }>();
}
