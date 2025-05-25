import { WebResponse } from "@/types";
import { api } from "@/utils/query";

export async function deleteChallengeAttachment(id: string) {
  return api
    .delete(`admin/challenges/${id}/attachment`)
    .json<WebResponse<never>>();
}
