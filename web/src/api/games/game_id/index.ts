import { ScoreRecord } from "@/models/game";
import { alova } from "@/utils/alova";
import { WebResponse } from "@/types";

export interface GetGameScoreboardRequest {
    id?: number;
    size?: number;
    page?: number;
}

export async function getGameScoreboard(request: GetGameScoreboardRequest) {
    return alova.Get<WebResponse<Array<ScoreRecord>>>(
        `/games/${request.id}/scoreboard`,
        {
            params: request,
            cacheFor: 30 * 1000,
        }
    );
}
