import { Metadata } from "@/models/media";
import { WebResponse } from "@/types";
import { api } from "@/utils/ky";

export async function getChallengeAttachmentMetadata(id: string) {
  return api
    .get(`challenges/${id}/attachment/metadata`)
    .json<WebResponse<Metadata>>();
}
