import { ChallengeMini } from "@/models/challenge";
import { Submission } from "@/models/submission";
import { WebResponse } from "@/types";
import { api, toSearchParams } from "@/utils/query";

export interface GetPlaygroundChallengesRequest {
  id?: string;
  title?: string;
  tags?: string;
  category?: number;
  is_dynamic?: boolean;
  page?: number;
  size?: number;
  sorts?: string;
}

export async function getPlaygroundChallenges(
  request: GetPlaygroundChallengesRequest
) {
  return api
    .get("challenges/playground", {
      searchParams: toSearchParams(request),
    })
    .json<WebResponse<Array<ChallengeMini>>>();
}

export interface GetChallengeStatusRequest {
  challenge_ids: Array<string>;
  user_id?: number;
  team_id?: number;
  game_id?: number;
}

export interface ChallengeStatus {
  is_solved?: boolean;
  solved_times?: number;
  pts?: number;
  bloods?: Array<Submission>;
}

export async function getChallengeStatus(request: GetChallengeStatusRequest) {
  return api
    .post("challenges/status", {
      json: request,
    })
    .json<WebResponse<Record<string, ChallengeStatus>>>();
}
