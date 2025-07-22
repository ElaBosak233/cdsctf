import type { Metadata } from "@/models/media";
import type { WebResponse } from "@/types";
import { api } from "@/utils/query";

export async function getChallengeAttachments(id: string) {
  return api
    .get(`challenges/${id}/attachments`)
    .json<WebResponse<Array<Metadata>>>();
}
