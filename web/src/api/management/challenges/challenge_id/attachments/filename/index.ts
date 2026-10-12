import { api } from "@/utils/query";

export async function deleteChallengeAttachment(
  id?: number,
  filename?: string
) {
  return api
    .delete(`management/challenges/${id}/attachments/${filename}`)
    .json<Record<string, never>>();
}
