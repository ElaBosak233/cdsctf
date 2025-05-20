import { Env } from "@/models/env";
import { WebResponse } from "@/types";
import { api, toSearchParams } from "@/utils/ky";

export interface GetEnvRequest {
  id?: string;
  game_id?: number;
  user_id?: number;
  team_id?: number;
  challenge_id?: string;
}

export async function getEnvs(request: GetEnvRequest) {
  return api
    .get("envs", {
      searchParams: toSearchParams(request),
    })
    .json<WebResponse<Array<Env>>>();
}

export interface CreateEnvRequest {
  game_id?: number;
  team_id?: number;
  challenge_id?: string;
}

export async function createEnv(request: CreateEnvRequest) {
  return api.post("envs", { json: request }).json<WebResponse<Env>>();
}
