import type { GameNoticeView } from "@/models/game_notice";
import { api, toSearchParams } from "@/utils/query";

export type GetGameNoticeRequest = {
  game_id?: number;
};

export async function getGameNotice(request: GetGameNoticeRequest) {
  return api
    .get(`games/${request.game_id}/notices`, {
      searchParams: toSearchParams(request),
    })
    .json<{ notices: GameNoticeView[]; total: number }>();
}
