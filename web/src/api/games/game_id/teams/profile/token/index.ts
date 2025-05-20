import { WebResponse } from "@/types";
import { api, toSearchParams } from "@/utils/ky";

export interface CreateTokenRequest {
  team_id?: number;
  game_id?: number;
}

export async function createToken(request: CreateTokenRequest) {
  return api
    .post(`games/${request.game_id}/teams/profile/token`, {
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
    .get(`games/${request.game_id}/teams/profile/token`, {
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
    .post(`games/${request.game_id}/teams/profile/token`, { json: request })
    .json<WebResponse<string>>();
}
