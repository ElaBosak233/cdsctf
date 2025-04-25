import { WebResponse } from "@/types";
import { alova } from "@/utils/alova";

export interface DeleteTeamAvatarRequest {
    game_id: number;
    team_id: number;
}

export function deleteTeamAvatar(request: DeleteTeamAvatarRequest) {
    return alova.Delete<WebResponse<never>>(
        `/admin/games/${request.game_id}/teams/${request.team_id}/avatar`
    );
}
