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
