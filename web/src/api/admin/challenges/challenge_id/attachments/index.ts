import type { Metadata } from "@/models/media";
import { api } from "@/utils/query";

export async function getChallengeAttachments(id: number) {
  return api
    .get(`admin/challenges/${id}/attachments`)
    .json<{ items: Metadata[]; total: number }>();
}
