import { Challenge } from "@/models/challenge";
import { WebResponse } from "@/types";
import { api } from "@/utils/query";

export interface GetChallengeRequest {
  id?: string;
}

export async function getChallenge(request: GetChallengeRequest) {
  return api.get(`challenges/${request.id}`).json<WebResponse<Challenge>>();
}
