import { Challenge } from "@/models/challenge";
import { WebResponse } from "@/types";
import { api } from "@/utils/query";

export interface GetChallengeRequest {
  id?: string;
}

export async function getChallenge(request: GetChallengeRequest) {
  return api
    .get(`admin/challenges/${request.id}`)
    .json<WebResponse<Challenge>>();
}

export interface UpdateChallengeRequest {
  id?: string | null;
  title?: string | null;
  tags?: Array<string> | null;
  description?: string | null;
  category?: number | null;
  has_attachment?: boolean | null;
  is_public?: boolean | null;
  is_dynamic?: boolean | null;
}

export async function updateChallenge(request: UpdateChallengeRequest) {
  return api
    .put(`admin/challenges/${request?.id}`, { json: request })
    .json<WebResponse<Challenge>>();
}

export interface DeleteChallengeRequest {
  id?: string;
}

export async function deleteChallenge(request: DeleteChallengeRequest) {
  return api
    .delete(`admin/challenges/${request.id}`)
    .json<WebResponse<never>>();
}
