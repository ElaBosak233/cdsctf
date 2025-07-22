import type { WebResponse } from "@/types";
import { api } from "@/utils/query";

export interface UpdateChallengeCheckerRequest {
  id?: string;
  checker?: string;
}

export async function updateChallengeChecker(
  request: UpdateChallengeCheckerRequest
) {
  return api
    .put(`admin/challenges/${request?.id}/checker`, { json: request })
    .json<WebResponse<never>>();
}
