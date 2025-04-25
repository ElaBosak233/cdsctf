import { WebResponse } from "@/types";
import { alova } from "@/utils/alova";

export interface CreateTokenRequest {
    team_id?: number;
    game_id?: number;
}

export async function createToken(request: CreateTokenRequest) {
    return alova.Post<WebResponse<string>>(
        `/admin/games/${request.game_id}/teams/${request.team_id}/token`,
        request
    );
}

export interface GetTokenRequest {
    team_id?: number;
    game_id?: number;
}

export async function getToken(request: GetTokenRequest) {
    return alova.Get<WebResponse<string>>(
        `/admin/games/${request.game_id}/teams/${request.team_id}/token`,
        {
            params: request,
        }
    );
}

export interface DeleteTokenRequest {
    team_id?: number;
    game_id?: number;
}

export async function deleteToken(request: DeleteTokenRequest) {
    return alova.Post<WebResponse<string>>(
        `/admin/games/${request.game_id}/teams/${request.team_id}/token`,
        request
    );
}
