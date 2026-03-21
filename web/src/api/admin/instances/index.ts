import { api } from "@/utils/query";

export interface CreateDebugInstanceRequest {
  challenge_id?: number;
}

export async function createDebugInstance(request: CreateDebugInstanceRequest) {
  return api
    .post("admin/instances", { json: request })
    .json<{ instance_id: string }>();
}
