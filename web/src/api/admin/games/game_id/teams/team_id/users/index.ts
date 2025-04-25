import { WebResponse } from "@/types";
import { alova } from "@/utils/alova";

export interface CreateTeamUserRequest {
    team_id?: number;
    game_id?: number;
    user_id?: number;
}

export async function createTeamUser(request: CreateTeamUserRequest) {
    return alova.Post<WebResponse<never>>(
        `/admin/games/${request.game_id}/teams/${request.team_id}/users`,
        request
    );
}

export interface DeleteTeamUserRequest {
    team_id?: number;
    game_id?: number;
    user_id?: number;
}

export async function deleteTeamUser(request: DeleteTeamUserRequest) {
    return alova.Delete<WebResponse<never>>(
        `/admin/games/${request.game_id}/teams/${request.team_id}/users/${request.user_id}`,
        request
    );
}
