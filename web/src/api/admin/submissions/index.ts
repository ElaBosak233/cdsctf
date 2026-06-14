import type { Status, Submission } from "@/models/submission";
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

export type UpdateSubmissionStatusRequest = {
  submission_id: number;
  status: Status;
};

export async function updateSubmissionStatus(
  request: UpdateSubmissionStatusRequest
) {
  return api
    .put(`admin/submissions/${request.submission_id}/status`, {
      json: { status: request.status },
    })
    .json<Submission>();
}
