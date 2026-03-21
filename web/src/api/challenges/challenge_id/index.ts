import type { Challenge } from "@/models/challenge";
import { api } from "@/utils/query";

export interface GetChallengeRequest {
  id?: number;
}

export async function getChallenge(request: GetChallengeRequest) {
  return api.get(`challenges/${request.id}`).json<{ challenge: Challenge }>();
}
