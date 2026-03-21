import type { Instance } from "@/models/challenge";
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
    .json<Record<string, never>>();
}
