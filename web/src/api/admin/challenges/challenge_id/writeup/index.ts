import type { Challenge } from "@/models/challenge";
import type { WebResponse } from "@/types";
import { api } from "@/utils/query";

export interface UpdateWriteupRequest {
  id?: number | null;
  writeup?: string | null;
}

export async function updateWriteup(request: UpdateWriteupRequest) {
  return api
    .put(`admin/challenges/${request?.id}/writeup`, { json: request })
    .json<WebResponse<Challenge>>();
}
