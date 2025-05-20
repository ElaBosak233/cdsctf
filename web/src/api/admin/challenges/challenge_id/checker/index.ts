import { WebResponse } from "@/types";
import { api } from "@/utils/ky";

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
