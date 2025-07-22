import type { WebResponse } from "@/types";
import { api, toSearchParams } from "@/utils/query";

export interface CreateTokenRequest {
  team_id?: number;
  game_id?: number;
}

export async function createToken(request: CreateTokenRequest) {
  return api
    .post(`admin/games/${request.game_id}/teams/${request.team_id}/token`, {
      json: request,
    })
    .json<WebResponse<string>>();
}

export interface GetTokenRequest {
  team_id?: number;
  game_id?: number;
}

export async function getToken(request: GetTokenRequest) {
  return api
    .get(`admin/games/${request.game_id}/teams/${request.team_id}/token`, {
      searchParams: toSearchParams(request),
    })
    .json<WebResponse<string>>();
}

export interface DeleteTokenRequest {
  team_id?: number;
  game_id?: number;
}

export async function deleteToken(request: DeleteTokenRequest) {
  return api
    .post(`admin/games/${request.game_id}/teams/${request.team_id}/token`, {
      json: request,
    })
    .json<WebResponse<string>>();
}
