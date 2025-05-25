import { GameMini } from "@/models/game";
import { WebResponse } from "@/types";
import { api, toSearchParams } from "@/utils/query";

export interface GetGameRequest {
  id?: number;
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
    .json<WebResponse<Array<GameMini>>>();
}
