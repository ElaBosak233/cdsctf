import { Metadata } from "@/models/media";
import { WebResponse } from "@/types";
import { alova } from "@/utils/alova";

export async function getChallengeAttachmentMetadata(id: string) {
    return alova.Get<WebResponse<Metadata>>(
        `/challenges/${id}/attachment/metadata`
    );
}
