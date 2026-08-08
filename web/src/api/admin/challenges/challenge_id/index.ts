import type { ChallengeDetail } from "@/models/challenge";
import { api } from "@/utils/query";

export type GetChallengeRequest = {
  id?: number;
};

export async function getChallenge(request: GetChallengeRequest) {
  return api
    .get(`admin/challenges/${request.id}`)
    .json<{ challenge: ChallengeDetail }>();
}

export type UpdateChallengeRequest = {
  id?: number | null;
  title?: string | null;
  tags?: Array<string> | null;
  description?: string | null;
  category?: number | null;
  has_attachment?: boolean | null;
  public?: boolean | null;
  has_instance?: boolean | null;
};

export async function updateChallenge(request: UpdateChallengeRequest) {
  return api
    .put(`admin/challenges/${request?.id}`, { json: request })
    .json<{ challenge: ChallengeDetail }>();
}

export type DeleteChallengeRequest = {
  id?: number;
};

export async function deleteChallenge(request: DeleteChallengeRequest) {
  return api
    .delete(`admin/challenges/${request.id}`)
    .json<Record<string, never>>();
}
