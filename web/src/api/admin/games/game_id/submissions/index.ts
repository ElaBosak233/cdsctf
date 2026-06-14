import type { Status, Submission } from "@/models/submission";
import { api, toSearchParams } from "@/utils/query";

export type GetSubmissionsRequest = {
  game_id: number;
  id?: number;
  team_id?: number;
  challenge_id?: number;
  status?: Status;
  page?: number;
  size?: number;
  sorts?: string;
};

export async function getSubmissions(request: GetSubmissionsRequest) {
  return api
    .get("admin/submissions", {
      searchParams: toSearchParams(request),
    })
    .json<{ submissions: Submission[]; total: number }>();
}
