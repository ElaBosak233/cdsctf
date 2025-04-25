import { GameNotice } from "@/models/game_notice";
import { WebResponse } from "@/types";
import { alova } from "@/utils/alova";

export interface GetGameNoticeRequest {
    game_id?: number;
}

export async function getGameNotice(request: GetGameNoticeRequest) {
    return alova.Get<WebResponse<Array<GameNotice>>>(
        `/games/${request.game_id}/notices`,
        {
            params: request,
        }
    );
}
