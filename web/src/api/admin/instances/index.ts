import { createInstance } from "@/api/instances";

export type CreateDebugInstanceRequest = {
  challenge_id?: number;
};

export async function createDebugInstance(request: CreateDebugInstanceRequest) {
  return createInstance({
    challenge_id: request.challenge_id,
    purpose: "debug",
  });
}
