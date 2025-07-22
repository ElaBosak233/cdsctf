import type { WebResponse } from "@/types";
import { api } from "@/utils/query";

export async function deleteChallengeAttachment(
  id?: string,
  filename?: string
) {
  return api
    .delete(`admin/challenges/${id}/attachments/${filename}`)
    .json<WebResponse<never>>();
}
