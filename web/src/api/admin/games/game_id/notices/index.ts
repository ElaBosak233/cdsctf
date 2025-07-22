import type { GameNotice } from "@/models/game_notice";
import type { WebResponse } from "@/types";
import { api } from "@/utils/query";

export interface CreateGameNoticeRequest {
  game_id?: number;
  title?: string;
  content?: string;
}

export async function createGameNotice(request: CreateGameNoticeRequest) {
  return api
    .post(`admin/games/${request.game_id}/notices`, { json: request })
    .json<WebResponse<GameNotice>>();
}

export interface DeleteGameNoticeRequest {
  id?: number;
  game_id?: number;
}

export async function deleteGameNotice(request: DeleteGameNoticeRequest) {
  return api
    .delete(`admin/games/${request.game_id}/notices/${request.id}`, {
      json: request,
    })
    .json<WebResponse<never>>();
}
