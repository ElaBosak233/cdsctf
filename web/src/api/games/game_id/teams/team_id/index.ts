import { UserMini } from "@/models/user";
import { WebResponse } from "@/types";
import { alova } from "@/utils/alova";

export interface JoinTeamRequest {
    game_id?: number;
    team_id?: number;
    token?: string;
}

export async function joinTeam(request: JoinTeamRequest) {
    return alova.Post<WebResponse<never>>(
        `/games/${request?.game_id}/teams/${request?.team_id}/join`,
        request
    );
}

export interface GetTeamMemberRequest {
    game_id?: number;
    team_id?: number;
}

export async function getTeamMembers(request: GetTeamMemberRequest) {
    return alova.Get<WebResponse<Array<UserMini>>>(
        `/games/${request.game_id}/teams/${request.team_id}/members`
    );
}
