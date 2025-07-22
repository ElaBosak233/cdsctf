import type { Submission } from "@/models/submission";
import type { WebResponse } from "@/types";
import { api, toSearchParams } from "@/utils/query";

export interface CreateSubmissionRequest {
  content?: string;
  challenge_id?: string;
  team_id?: number;
  game_id?: number;
}

export async function createSubmission(request: CreateSubmissionRequest) {
  return api
    .post("submissions", {
      json: request,
    })
    .json<WebResponse<Submission>>();
}

export interface GetSubmissionRequest {
  id?: number;
  content?: string;
  status?: number;
  user_id?: number;
  is_detailed?: boolean;
  challenge_id?: string;
  team_id?: number;
  game_id?: number;
  size?: number;
  page?: number;
  sorts?: string;

  is_desensitized?: boolean;
}

export async function getSubmission(request: GetSubmissionRequest) {
  return api
    .get("submissions", {
      searchParams: toSearchParams(request),
    })
    .json<WebResponse<Array<Submission>>>();
}
