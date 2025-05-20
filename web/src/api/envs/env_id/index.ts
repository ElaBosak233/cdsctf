import { Env } from "@/models/challenge";
import { WebResponse } from "@/types";
import { api } from "@/utils/ky";

export interface StopEnvRequest {
  id: string;
}

export async function stopEnv(request: StopEnvRequest) {
  return api
    .post(`envs/${request.id}/stop`, { json: request })
    .json<WebResponse<unknown>>();
}

export interface RenewEnvRequest {
  id: string;
  team_id?: number;
  game_id?: number;
}

export async function renewEnv(request: RenewEnvRequest) {
  return api
    .post(`envs/${request.id}/renew`, { json: request })
    .json<WebResponse<Env>>();
}
