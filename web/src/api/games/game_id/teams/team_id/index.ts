import type { UserMini } from "@/models/user";
import type { WebResponse } from "@/types";
import { api } from "@/utils/query";

export interface JoinTeamRequest {
  game_id?: number;
  team_id?: number;
  token?: string;
}

export async function joinTeam(request: JoinTeamRequest) {
  return api
    .post(`games/${request?.game_id}/teams/${request?.team_id}/join`, {
      json: request,
    })
    .json<WebResponse<never>>();
}

export interface GetTeamMemberRequest {
  game_id?: number;
  team_id?: number;
}

export async function getTeamMembers(request: GetTeamMemberRequest) {
  return api
    .get(`games/${request.game_id}/teams/${request.team_id}/members`)
    .json<WebResponse<Array<UserMini>>>();
}
