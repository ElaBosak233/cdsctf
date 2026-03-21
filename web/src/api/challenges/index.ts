import type { ChallengeMini } from "@/models/challenge";
import type { Submission } from "@/models/submission";
import { api, toSearchParams } from "@/utils/query";

export interface ListChallengesRequest {
  id?: number;
  title?: string;
  tag?: string;
  category?: number;
  has_instance?: boolean;
  page?: number;
  size?: number;
  sorts?: string;
}

export async function listChallenges(request: ListChallengesRequest) {
  return api
    .get("challenges", {
      searchParams: toSearchParams(request),
    })
    .json<{ challenges: ChallengeMini[]; total: number }>();
}

export interface QueryChallengeStatusRequest {
  challenge_ids: Array<number>;
  user_id?: number;
  team_id?: number;
  game_id?: number;
}

export interface ChallengeStatus {
  solved?: boolean;
  solved_times?: number;
  pts?: number;
  bloods?: Array<Submission>;
}

export async function queryChallengeStatus(
  request: QueryChallengeStatusRequest
) {
  return api
    .post("challenges/status", {
      json: request,
    })
    .json<{ statuses: Record<string, ChallengeStatus> }>();
}
