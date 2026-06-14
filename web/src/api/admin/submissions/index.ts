import type { Status } from "@/models/submission";
import { api } from "@/utils/query";

export type DebugCreateSubmissionRequest = {
  content: string;
  challenge_id: number;
};

export type DebugCreateSubmissionResponse = {
  status: Status;
};

export async function debugCreateSubmission(
  request: DebugCreateSubmissionRequest
) {
  return api
    .post("admin/submissions/debug", {
      json: request,
    })
    .json<DebugCreateSubmissionResponse>();
}
