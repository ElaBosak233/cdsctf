import type { State, Team } from "@/models/team";
import type { WebResponse } from "@/types";
import { api } from "@/utils/query";

export interface UpdateTeamRequest {
  team_id: number;
  game_id: number;
  name?: string | null;
  email?: string | null;
  slogan?: string | null;
  description?: string | null;
  state?: State | null;
}

export async function updateTeam(request: UpdateTeamRequest) {
  return api
    .put(`admin/games/${request.game_id}/teams/${request.team_id}`, {
      json: request,
    })
    .json<WebResponse<Team>>();
}

export interface DeleteTeamRequest {
  team_id?: number;
  game_id?: number;
}

export async function deleteTeam(request: DeleteTeamRequest) {
  return api
    .delete(`admin/games/${request.game_id}/teams/${request.team_id}`, {
      json: request,
    })
    .json<WebResponse<never>>();
}
