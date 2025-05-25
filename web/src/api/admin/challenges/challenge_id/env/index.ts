import { Env } from "@/models/challenge";
import { WebResponse } from "@/types";
import { api } from "@/utils/query";

export interface UpdateChallengeEnvRequest {
  id?: string;
  env?: Env;
}

export async function updateChallengeEnv(request: UpdateChallengeEnvRequest) {
  return api
    .put(`admin/challenges/${request?.id}/env`, { json: request })
    .json<WebResponse<never>>();
}
