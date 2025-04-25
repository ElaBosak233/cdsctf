import { Game } from "@/models/game";
import { WebResponse } from "@/types";
import { alova } from "@/utils/alova";

export interface GetGameRequest {
    id?: number;
    title?: string;
    sorts?: string;
    page?: number;
    size?: number;
}

export async function getGames(request: GetGameRequest) {
    return alova.Get<WebResponse<Array<Game>>>("/games", {
        params: request,
    });
}
