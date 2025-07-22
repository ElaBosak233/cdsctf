import type { WebResponse } from "@/types";
import { api } from "@/utils/query";

export interface DeleteTeamAvatarRequest {
  game_id: number;
  team_id: number;
}

export function deleteTeamAvatar(request: DeleteTeamAvatarRequest) {
  return api
    .delete(`admin/games/${request.game_id}/teams/${request.team_id}/avatar`)
    .json<WebResponse<never>>();
}
