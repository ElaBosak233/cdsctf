import type { GameChallenge } from "@/models/game_challenge";
import type { WebResponse } from "@/types";
import { api, toSearchParams } from "@/utils/query";

export interface GetGameChallengeRequest {
  game_id?: number;
  challenge_id?: number;
  category?: number;
}

export async function getGameChallenges(request: GetGameChallengeRequest) {
  return api
    .get(`games/${request.game_id}/challenges`, {
      searchParams: toSearchParams(request),
    })
    .json<WebResponse<Array<GameChallenge>>>();
}
