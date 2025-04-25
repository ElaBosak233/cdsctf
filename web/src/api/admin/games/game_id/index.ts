import { Game } from "@/models/game";
import { WebResponse } from "@/types";
import { alova } from "@/utils/alova";

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
    return alova.Put<WebResponse<Game>>(`/admin/games/${request.id}`, request);
}

export interface DeleteGameRequest {
    id?: number;
}

export async function deleteGame(request: DeleteGameRequest) {
    return alova.Delete<WebResponse<never>>(`/admin/games/${request.id}`);
}
