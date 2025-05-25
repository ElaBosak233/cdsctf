import { WebResponse } from "@/types";
import { api } from "@/utils/query";

export interface LeaveTeamRequest {
  game_id?: number;
  team_id?: number;
}

export async function leaveTeam(request: LeaveTeamRequest) {
  return api
    .delete(`games/${request?.game_id}/teams/profile/users/leave`, {
      json: request,
    })
    .json<WebResponse<never>>();
}
