import type { Challenge } from "@/models/challenge";
import type { WebResponse } from "@/types";
import { api, toSearchParams } from "@/utils/query";

export interface GetChallengesRequest {
  id?: number;
  title?: string;
  description?: string;
  category?: number;
  is_public?: boolean;
  is_dynamic?: boolean;
  page?: number;
  size?: number;
  sorts?: string;
}

export async function getChallenges(request: GetChallengesRequest) {
  return api
    .get("admin/challenges", {
      searchParams: toSearchParams(request),
    })
    .json<WebResponse<Array<Challenge>>>();
}

export interface CreateChallengeRequest {
  title?: string;
  description?: string;
  category?: number;
  is_public?: boolean;
  is_dynamic?: boolean;
  has_attachment?: boolean;
}

export async function createChallenge(request: CreateChallengeRequest) {
  return api
    .post("admin/challenges", { json: request })
    .json<WebResponse<Challenge>>();
}
