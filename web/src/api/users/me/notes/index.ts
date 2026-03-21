import type { Note } from "@/models/note";
import { api, toSearchParams } from "@/utils/query";

export interface GetNotesRequest {
  challenge_id?: number;
  size?: number;
  page?: number;
  sorts?: string;
}

export async function getMyNotes(request: GetNotesRequest) {
  return api
    .get(`users/me/notes`, {
      searchParams: toSearchParams(request),
    })
    .json<{ items: Note[]; total: number }>();
}

export interface SaveNoteRequest {
  content: string;
  challenge_id: number;
  public: boolean;
}

export async function saveMyNote(request: SaveNoteRequest) {
  return api.post(`users/me/notes`, { json: request }).json<{ note: Note }>();
}
