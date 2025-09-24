import type { Metadata } from "@/models/media";
import type { WebResponse } from "@/types";
import { api } from "@/utils/query";

export async function getChallengeAttachments(id: number) {
  return api
    .get(`admin/challenges/${id}/attachments`)
    .json<WebResponse<Array<Metadata>>>();
}
