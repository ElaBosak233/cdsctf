import type { GameChallenge } from "@/models/game_challenge";
import type { WebResponse } from "@/types";
import { api } from "@/utils/query";

export interface UpdateGameChallengeRequest {
  game_id?: number;
  challenge_id?: number;
  is_enabled?: boolean;
  max_pts?: number;
  min_pts?: number;
  difficulty?: number;
  bonus_ratios?: Array<number>;
  frozen_at?: number | null;
}

export async function updateGameChallenge(request: UpdateGameChallengeRequest) {
  return api
    .put(`admin/games/${request.game_id}/challenges/${request.challenge_id}`, {
      json: request,
    })
    .json<WebResponse<GameChallenge>>();
}

export interface DeleteGameChallengeRequest {
  challenge_id?: number;
  game_id?: number;
}

export async function deleteGameChallenge(request: DeleteGameChallengeRequest) {
  return api
    .delete(
      `admin/games/${request.game_id}/challenges/${request.challenge_id}`,
      { json: request }
    )
    .json<WebResponse<never>>();
}
