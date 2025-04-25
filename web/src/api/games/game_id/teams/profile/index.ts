import { Team } from "@/models/team";
import { WebResponse } from "@/types";
import { alova } from "@/utils/alova";

export interface GetTeamProfile {
    game_id?: number;
}

export async function getTeamProfile(request: GetTeamProfile) {
    return alova.Get<WebResponse<Team>>(
        `/games/${request.game_id}/teams/profile`
    );
}

export interface UpdateTeamRequest {
    id: number;
    game_id: number;
    name?: string | null;
    email?: string | null;
    slogan?: string | null;
    description?: string | null;
}

export async function updateTeam(request: UpdateTeamRequest) {
    return alova.Put<WebResponse<Team>>(
        `/games/${request.game_id}/teams/profile`,
        request
    );
}

export interface DeleteTeamRequest {
    team_id?: number;
    game_id?: number;
}

export async function deleteTeam(request: DeleteTeamRequest) {
    return alova.Delete<WebResponse<never>>(
        `/games/${request.game_id}/teams/profile`,
        request
    );
}

export interface SetTeamReadyRequest {
    id?: number;
    game_id?: number;
}

export async function setTeamReady(request: SetTeamReadyRequest) {
    return alova.Post<WebResponse<never>>(
        `/games/${request.game_id}/teams/profile/ready`,
        request
    );
}
