import type { GameMini } from "@/models/game";
import { api, toSearchParams } from "@/utils/query";

export interface GetGameRequest {
  title?: string;
  sorts?: string;
  page?: number;
  size?: number;
}

export async function getGames(request: GetGameRequest) {
  return api
    .get("games", {
      searchParams: toSearchParams(request),
    })
    .json<{ games: GameMini[]; total: number }>();
}
