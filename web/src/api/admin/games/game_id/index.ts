import type { Game } from "@/models/game";
import type { WebResponse } from "@/types";
import { api } from "@/utils/query";

export interface UpdateGameRequest {
  id?: number;
  title?: string;
  sketch?: string | null;
  description?: string | null;
  is_enabled?: boolean;
  is_public?: boolean;
  is_need_write_up?: boolean;
  member_limit_min?: number;
  member_limit_max?: number;
  started_at?: number;
  frozen_at?: number;
  ended_at?: number;
}

export async function updateGame(request: UpdateGameRequest) {
  return api
    .put(`admin/games/${request.id}`, { json: request })
    .json<WebResponse<Game>>();
}

export interface DeleteGameRequest {
  id?: number;
}

export async function deleteGame(request: DeleteGameRequest) {
  return api.delete(`admin/games/${request.id}`).json<WebResponse<never>>();
}
