import type { ChallengeMini } from "@/models/challenge";
import type { Submission } from "@/models/submission";
import { api, toSearchParams } from "@/utils/query";

export interface GetPlaygroundChallengesRequest {
  id?: string;
  title?: string;
  tag?: string;
  category?: number;
  has_instance?: boolean;
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
    .json<{ items: ChallengeMini[]; total: number }>();
}

export interface GetChallengeStatusRequest {
  challenge_ids: Array<number>;
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
    .json<{ statuses: Record<string, ChallengeStatus> }>();
}
