import { WebResponse } from "@/types";
import { api } from "@/utils/query";

export interface CreateTeamUserRequest {
  team_id?: number;
  game_id?: number;
  user_id?: number;
}

export async function createTeamUser(request: CreateTeamUserRequest) {
  return api
    .post(`admin/games/${request.game_id}/teams/${request.team_id}/users`, {
      json: request,
    })
    .json<WebResponse<never>>();
}

export interface DeleteTeamUserRequest {
  team_id?: number;
  game_id?: number;
  user_id?: number;
}

export async function deleteTeamUser(request: DeleteTeamUserRequest) {
  return api
    .delete(
      `admin/games/${request.game_id}/teams/${request.team_id}/users/${request.user_id}`,
      { json: request }
    )
    .json<WebResponse<never>>();
}
