import { Metadata } from "@/models/media";
import { WebResponse } from "@/types";
import { api } from "@/utils/query";

export async function getChallengeAttachments(id: string) {
  return api
    .get(`admin/challenges/${id}/attachments`)
    .json<WebResponse<Array<Metadata>>>();
}
