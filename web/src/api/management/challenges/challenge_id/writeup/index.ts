import type { ChallengeDetail } from "@/models/challenge";
import { api } from "@/utils/query";

export type UpdateWriteupRequest = {
  id?: number | null;
  writeup?: string | null;
};

export async function updateWriteup(request: UpdateWriteupRequest) {
  return api
    .patch(`management/challenges/${request?.id}/writeup`, { json: request })
    .json<{ challenge: ChallengeDetail }>();
}
