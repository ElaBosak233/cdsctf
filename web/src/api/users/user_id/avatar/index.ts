import type { Metadata } from "@/models/media";
import type { WebResponse } from "@/types";
import { api } from "@/utils/query";

export async function getUserAvatarMetadata(id: number) {
  return api.get(`users/${id}/avatar/metadata`).json<WebResponse<Metadata>>();
}
