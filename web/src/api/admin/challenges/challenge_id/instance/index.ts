import type { Instance } from "@/models/challenge";
import type { WebResponse } from "@/types";
import { api } from "@/utils/query";

export interface UpdateChallengeInstanceRequest {
  id?: number;
  instance?: Instance;
}

export async function updateChallengeInstance(
  request: UpdateChallengeInstanceRequest
) {
  return api
    .put(`admin/challenges/${request?.id}/instance`, { json: request })
    .json<WebResponse<never>>();
}
