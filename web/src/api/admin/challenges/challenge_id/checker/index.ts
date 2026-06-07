import { api } from "@/utils/query";

export type UpdateCheckerRequest = {
  id?: number;
  checker?: string;
}

export async function updateChallengeChecker(request: UpdateCheckerRequest) {
  return api
    .put(`admin/challenges/${request?.id}/checker`, { json: request })
    .json<Record<string, never>>();
}

export type LintCheckerRequest = {
  id?: number;
  checker?: string;
}

export type DiagnosticMarker = {
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
    .json<{ markers: DiagnosticMarker[] }>();
}
