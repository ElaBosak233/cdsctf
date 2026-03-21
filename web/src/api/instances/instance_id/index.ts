import type { Instance } from "@/models/challenge";
import { api } from "@/utils/query";

export interface StopInstanceRequest {
  id: string;
}

export async function stopInstance(request: StopInstanceRequest) {
  return api
    .post(`instances/${request.id}/stop`, { json: request })
    .json<unknown>();
}

export interface RenewInstanceRequest {
  id: string;
  team_id?: number;
  game_id?: number;
}

export async function renewInstance(request: RenewInstanceRequest) {
  return api
    .post(`instances/${request.id}/renew`, { json: request })
    .json<Instance>();
}
