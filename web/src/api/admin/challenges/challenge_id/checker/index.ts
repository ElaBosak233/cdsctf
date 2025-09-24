import type { WebResponse } from "@/types";
import { api } from "@/utils/query";

export interface UpdateCheckerRequest {
  id?: number;
  checker?: string;
}

export async function updateChallengeChecker(request: UpdateCheckerRequest) {
  return api
    .put(`admin/challenges/${request?.id}/checker`, { json: request })
    .json<WebResponse<never>>();
}

export interface LintCheckerRequest {
  id?: number;
  checker?: string;
}

export interface DiagnosticMarker {
  start_line: number;
  start_column: number;
  end_line: number;
  end_column: number;
  kind: "error" | "warning";
  message: string;
}

export async function lintChallengeChecker(request: LintCheckerRequest) {
  return api
    .post(`admin/challenges/${request?.id}/checker/lint`, { json: request })
    .json<WebResponse<Array<DiagnosticMarker>>>();
}
