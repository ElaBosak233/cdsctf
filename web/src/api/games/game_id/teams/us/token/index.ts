import { api, toSearchParams } from "@/utils/query";

export type CreateTokenRequest = {
  team_id?: number;
  game_id?: number;
}

export async function createToken(request: CreateTokenRequest) {
  return api
    .post(`games/${request.game_id}/teams/us/token`, {
      json: request,
    })
    .json<{ token: string | null }>();
}

export type GetTokenRequest = {
  team_id?: number;
  game_id?: number;
}

export async function getToken(request: GetTokenRequest) {
  return api
    .get(`games/${request.game_id}/teams/us/token`, {
      searchParams: toSearchParams(request),
    })
    .json<{ token: string | null }>();
}

export type DeleteTokenRequest = {
  team_id?: number;
  game_id?: number;
}

export async function deleteToken(request: DeleteTokenRequest) {
  return api
    .post(`games/${request.game_id}/teams/us/token`, { json: request })
    .json<{ token: string | null }>();
}
