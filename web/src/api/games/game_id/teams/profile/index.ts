import type { Team } from "@/models/team";
import type { WebResponse } from "@/types";
import { api } from "@/utils/query";

export interface GetTeamProfile {
  game_id?: number;
}

export async function getTeamProfile(request: GetTeamProfile) {
  return api
    .get(`games/${request.game_id}/teams/profile`)
    .json<WebResponse<Team>>();
}

export interface UpdateTeamRequest {
  id: number;
  game_id: number;
  name?: string | null;
  email?: string | null;
  slogan?: string | null;
  description?: string | null;
}

export async function updateTeam(request: UpdateTeamRequest) {
  return api
    .put(`games/${request.game_id}/teams/profile`, { json: request })
    .json<WebResponse<Team>>();
}

export interface DeleteTeamRequest {
  team_id?: number;
  game_id?: number;
}

export async function deleteTeam(request: DeleteTeamRequest) {
  return api
    .delete(`games/${request.game_id}/teams/profile`, {
      json: request,
    })
    .json<WebResponse<never>>();
}

export interface SetTeamReadyRequest {
  id?: number;
  game_id?: number;
}

export async function setTeamReady(request: SetTeamReadyRequest) {
  return api
    .post(`games/${request.game_id}/teams/profile/ready`, { json: request })
    .json<WebResponse<never>>();
}
