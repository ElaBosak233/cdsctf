import { State, Team } from "@/models/team";
import { WebResponse } from "@/types";
import { alova } from "@/utils/alova";

export interface UpdateTeamRequest {
    team_id: number;
    game_id: number;
    name?: string | null;
    email?: string | null;
    slogan?: string | null;
    description?: string | null;
    state?: State | null;
}

export async function updateTeam(request: UpdateTeamRequest) {
    return alova.Put<WebResponse<Team>>(
        `/admin/games/${request.game_id}/teams/${request.team_id}`,
        request
    );
}

export interface DeleteTeamRequest {
    team_id?: number;
    game_id?: number;
}

export async function deleteTeam(request: DeleteTeamRequest) {
    return alova.Delete<WebResponse<never>>(
        `/admin/games/${request.game_id}/teams/${request.team_id}`,
        request
    );
}
