import type { GameView, ScoreboardEntry } from "@/models/game";
import { api, toSearchParams } from "@/utils/query";

export type GetGameRequest = {
  id?: number;
};

export async function getGame(request: GetGameRequest) {
  return api.get(`games/${request.id}`).json<{ game: GameView }>();
}

export type GetGameScoreboardRequest = {
  id?: number;
  size?: number;
  page?: number;
};

export async function getGameScoreboard(request: GetGameScoreboardRequest) {
  return api
    .get(`games/${request.id}/scoreboard`, {
      searchParams: toSearchParams(request),
    })
    .json<{ records: ScoreboardEntry[]; total: number }>();
}
