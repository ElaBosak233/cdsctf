import type { Game } from "@/models/game";
import type { WebResponse } from "@/types";
import { api, toSearchParams } from "@/utils/query";

export interface GetGamesRequest {
  id?: number;
  title?: string;
  enabled?: boolean;
  sorts?: string;
  page?: number;
  size?: number;
}

export async function getGames(request: GetGamesRequest) {
  return api
    .get("admin/games", {
      searchParams: toSearchParams(request),
    })
    .json<WebResponse<Array<Game>>>();
}

export interface CreateGameRequest {
  title?: string;
  sketch?: string;
  description?: string;
  enabled?: boolean;
  public?: boolean;
  writeup_required?: boolean;
  member_limit_min?: number;
  member_limit_max?: number;
  started_at?: number;
  ended_at?: number;
}

export async function createGame(request: CreateGameRequest) {
  return api
    .post("admin/games", {
      json: request,
    })
    .json<WebResponse<Game>>();
}
