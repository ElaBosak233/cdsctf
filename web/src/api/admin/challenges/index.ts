import type { Challenge } from "@/models/challenge";
import { api, toSearchParams } from "@/utils/query";

export interface GetChallengesRequest {
  id?: number;
  title?: string;
  description?: string;
  category?: number;
  public?: boolean;
  has_instance?: boolean;
  page?: number;
  size?: number;
  sorts?: string;
}

export async function getChallenges(request: GetChallengesRequest) {
  return api
    .get("admin/challenges", {
      searchParams: toSearchParams(request),
    })
    .json<{ items: Challenge[]; total: number }>();
}

export interface CreateChallengeRequest {
  title?: string;
  description?: string;
  category?: number;
  public?: boolean;
  has_instance?: boolean;
  has_attachment?: boolean;
}

export async function createChallenge(request: CreateChallengeRequest) {
  return api
    .post("admin/challenges", { json: request })
    .json<{ challenge: Challenge }>();
}
