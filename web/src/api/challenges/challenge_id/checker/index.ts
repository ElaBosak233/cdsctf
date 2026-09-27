import { api } from "@/utils/query";

export type UpdateCheckerRequest = {
  id?: number;
  checker?: string;
};

export async function updateChallengeChecker(request: UpdateCheckerRequest) {
  const { id, ...body } = request;
  return api
    .put(`challenges/${id}/checker`, { json: body })
    .json<Record<string, never>>();
}

export type LintCheckerRequest = {
  id?: number;
  checker?: string;
};

export type DiagnosticMarker = {
  start_line: number;
  start_column: number;
  end_line: number;
  end_column: number;
  kind: "error" | "warning";
  message: string;
};

export async function lintChallengeChecker(request: LintCheckerRequest) {
  const { id, ...body } = request;
  return api
    .post(`challenges/${id}/checker/lint`, { json: body })
    .json<{ markers: DiagnosticMarker[] }>();
}
