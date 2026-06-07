import type { UserMini } from "@/models/user";
import { api } from "@/utils/query";

export type JoinTeamRequest = {
  game_id?: number;
  team_id?: number;
  token?: string;
}

export async function joinTeam(request: JoinTeamRequest) {
  return api
    .post(`games/${request?.game_id}/teams/${request?.team_id}/join`, {
      json: request,
    })
    .json<Record<string, never>>();
}

export type GetTeamMemberRequest = {
  game_id?: number;
  team_id?: number;
}

export async function getTeamMembers(request: GetTeamMemberRequest) {
  return api
    .get(`games/${request.game_id}/teams/${request.team_id}/members`)
    .json<{ users: UserMini[]; total: number }>();
}
