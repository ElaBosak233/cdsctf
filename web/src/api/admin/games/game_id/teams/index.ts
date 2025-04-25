import { State, Team } from "@/models/team";
import { WebResponse } from "@/types";
import { alova } from "@/utils/alova";

export interface GetTeamRequest {
    id?: number;
    game_id?: number;
    user_id?: number;
    name?: string;
    state?: State;
    page?: number;
    size?: number;
    sorts?: string;
}

export async function getTeams(request: GetTeamRequest) {
    return alova.Get<WebResponse<Array<Team>>>(
        `/admin/games/${request.game_id}/teams`,
        {
            params: request,
        }
    );
}

export interface CreateTeamRequest {
    game_id?: number;
    name?: string;
    email?: string;
    slogan?: string;
    description?: string;
}

export async function createTeam(request: CreateTeamRequest) {
    return alova.Post<WebResponse<Team>>(
        `/admin/games/${request.game_id}/teams`,
        request
    );
}
