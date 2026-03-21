import type { GameNotice } from "@/models/game_notice";
import { api, toSearchParams } from "@/utils/query";

export interface GetGameNoticeRequest {
  game_id?: number;
}

export async function getGameNotice(request: GetGameNoticeRequest) {
  return api
    .get(`games/${request.game_id}/notices`, {
      searchParams: toSearchParams(request),
    })
    .json<{ items: GameNotice[]; total: number }>();
}
