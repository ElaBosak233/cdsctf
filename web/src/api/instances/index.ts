import type { Instance } from "@/models/instance";
import type { WebResponse } from "@/types";
import { api, toSearchParams } from "@/utils/query";

export interface GetInstanceRequest {
  id?: string;
  game_id?: number;
  user_id?: number;
  team_id?: number;
  challenge_id?: number;
}

export async function getInstances(request: GetInstanceRequest) {
  return api
    .get("instances", {
      searchParams: toSearchParams(request),
    })
    .json<WebResponse<Array<Instance>>>();
}

export interface CreateInstanceRequest {
  game_id?: number;
  team_id?: number;
  challenge_id?: number;
}

export async function createInstance(request: CreateInstanceRequest) {
  return api.post("instances", { json: request }).json<WebResponse<Instance>>();
}
