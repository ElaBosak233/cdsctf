import { Game } from "@/models/game";
import { WebResponse } from "@/types";
import { api, toSearchParams } from "@/utils/ky";

export interface GetGameRequest {
  id?: number;
  title?: string;
  is_enabled?: boolean;
  sorts?: string;
  page?: number;
  size?: number;
}

export async function getGames(request: GetGameRequest) {
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
  is_enabled?: boolean;
  is_public?: boolean;
  is_need_write_up?: boolean;
  member_limit_min?: number;
  member_limit_max?: number;
  started_at?: number;
  ended_at?: number;
}

export async function createGame(request: CreateGameRequest) {
  return api
    .post("admin/games", {
      searchParams: toSearchParams(request),
    })
    .json<WebResponse<Game>>();
}
