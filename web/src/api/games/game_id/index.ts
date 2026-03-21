import type { Game, ScoreRecord } from "@/models/game";
import { api, toSearchParams } from "@/utils/query";

export interface GetGameRequest {
  id?: number;
}

export async function getGame(request: GetGameRequest) {
  return api.get(`games/${request.id}`).json<{ game: Game }>();
}

export interface GetGameScoreboardRequest {
  id?: number;
  size?: number;
  page?: number;
}

export async function getGameScoreboard(request: GetGameScoreboardRequest) {
  return api
    .get(`games/${request.id}/scoreboard`, {
      searchParams: toSearchParams(request),
    })
    .json<{ records: ScoreRecord[]; total: number }>();
}
