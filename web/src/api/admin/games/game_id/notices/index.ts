import { GameNotice } from "@/models/game_notice";
import { WebResponse } from "@/types";
import { alova } from "@/utils/alova";

export interface CreateGameNoticeRequest {
    game_id?: number;
    title?: string;
    content?: string;
}

export async function createGameNotice(request: CreateGameNoticeRequest) {
    return alova.Post<WebResponse<GameNotice>>(
        `/admin/games/${request.game_id}/notices`,
        request
    );
}

export interface DeleteGameNoticeRequest {
    id?: number;
    game_id?: number;
}

export async function deleteGameNotice(request: DeleteGameNoticeRequest) {
    return alova.Delete<WebResponse<never>>(
        `/admin/games/${request.game_id}/notices/${request.id}`,
        request
    );
}
