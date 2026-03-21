import type { Instance } from "@/models/instance";
import { api } from "@/utils/query";

export interface CreateDebugInstanceRequest {
  challenge_id?: number;
}

export async function createDebugInstance(request: CreateDebugInstanceRequest) {
  return api
    .post("admin/instances", { json: request })
    .json<Instance>();
}
