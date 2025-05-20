import { State, Team } from "@/models/team";
import { WebResponse } from "@/types";
import { api, toSearchParams } from "@/utils/ky";

export interface GetTeamRequest {
  id?: number;
  game_id?: number;
  user_id?: number;
  name?: string;
  state?: State;
  page?: number;
  size?: number;
  sorts?: string;
}

export async function getTeams(request: GetTeamRequest) {
  return api
    .get(`admin/games/${request.game_id}/teams`, {
      searchParams: toSearchParams(request),
    })
    .json<WebResponse<Array<Team>>>();
}

export interface CreateTeamRequest {
  game_id?: number;
  name?: string;
  email?: string;
  slogan?: string;
  description?: string;
}

export async function createTeam(request: CreateTeamRequest) {
  return api
    .post(`admin/games/${request.game_id}/teams`, { json: request })
    .json<WebResponse<Team>>();
}
