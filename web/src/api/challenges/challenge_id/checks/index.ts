import type { Status } from "@/models/submission";
import { api } from "@/utils/query";

export type CreateChallengeCheckRequest = {
  id: number;
  content: string;
};

export async function createChallengeCheck(
  request: CreateChallengeCheckRequest
) {
  return api
    .post(`challenges/${request.id}/checks`, {
      json: { content: request.content },
    })
    .json<{ status: Status }>();
}
