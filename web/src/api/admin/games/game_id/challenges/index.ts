import { GameChallenge } from "@/models/game_challenge";
import { WebResponse } from "@/types";
import { api, toSearchParams } from "@/utils/query";

export interface GetGameChallengeRequest {
  game_id?: number;
  challenge_id?: string;
  category?: number;
  is_enabled?: boolean;
}

export async function getGameChallenges(request: GetGameChallengeRequest) {
  return api
    .get(`admin/games/${request.game_id}/challenges`, {
      searchParams: toSearchParams(request),
    })
    .json<WebResponse<Array<GameChallenge>>>();
}

export interface CreateGameChallengeRequest {
  game_id?: number;
  challenge_id?: string;
  is_enabled?: boolean;
  max_pts?: number;
  min_pts?: number;
  difficulty?: number;
  bonus_ratios?: Array<number>;
}

export async function createGameChallenge(request: CreateGameChallengeRequest) {
  return api
    .post(`admin/games/${request.game_id}/challenges`, {
      json: request,
    })
    .json<WebResponse<GameChallenge>>();
}
