import { Challenge } from "@/models/challenge";
import { WebResponse } from "@/types";
import { api, toSearchParams } from "@/utils/ky";

export interface GetChallengeRequest {
  id?: string;
  title?: string;
  description?: string;
  category?: number;
  is_public?: boolean;
  is_dynamic?: boolean;
  page?: number;
  size?: number;
  sorts?: string;
}

export async function getChallenges(request: GetChallengeRequest) {
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
