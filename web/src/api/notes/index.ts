import type { Note } from "@/models/note";
import { api, toSearchParams } from "@/utils/query";

export interface GetNotesRequest {
  user_id?: number;
  challenge_id?: number;
  size?: number;
  page?: number;
  sorts?: string;
}

export async function getNotes(request: GetNotesRequest) {
  return api
    .get(`notes`, {
      searchParams: toSearchParams(request),
    })
    .json<{ items: Note[]; total: number }>();
}
