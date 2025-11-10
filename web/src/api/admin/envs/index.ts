import type { Env } from "@/models/env";
import type { WebResponse } from "@/types";
import { api } from "@/utils/query";

export interface CreateEnvRequest {
  challenge_id?: number;
}

export async function createEnv(request: CreateEnvRequest) {
  return api.post("admin/envs", { json: request }).json<WebResponse<Env>>();
}
