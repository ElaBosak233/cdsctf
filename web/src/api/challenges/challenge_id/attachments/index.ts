import type { Metadata } from "@/models/media";
import { api } from "@/utils/query";

export async function getChallengeAttachments(id: number) {
  return api
    .get(`challenges/${id}/attachments`)
    .json<{ attachments: Metadata[]; total: number }>();
}
