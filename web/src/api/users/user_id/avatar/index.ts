import { Metadata } from "@/models/media";
import { WebResponse } from "@/types";
import { alova } from "@/utils/alova";

export async function getUserAvatarMetadata(id: number) {
    return alova.Get<WebResponse<Metadata>>(`/users/${id}/avatar/metadata`);
}
