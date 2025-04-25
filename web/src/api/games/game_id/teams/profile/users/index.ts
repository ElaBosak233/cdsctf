import { WebResponse } from "@/types";
import { alova } from "@/utils/alova";

export interface LeaveTeamRequest {
    game_id?: number;
    team_id?: number;
}

export async function leaveTeam(request: LeaveTeamRequest) {
    return alova.Delete<WebResponse<never>>(
        `/games/${request?.game_id}/teams/profile/users/leave`,
        request
    );
}
