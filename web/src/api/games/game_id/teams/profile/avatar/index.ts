import type { WebResponse } from "@/types";
import { api } from "@/utils/query";

export interface DeleteTeamAvatarRequest {
  game_id: number;
  team_id: number;
}

export function deleteTeamAvatar(request: DeleteTeamAvatarRequest) {
  return api
    .delete(`games/${request.game_id}/teams/profile/avatar`)
    .json<WebResponse<never>>();
}
