import type { Game } from "@/models/game";
import { api } from "@/utils/query";

export type GetGameRequest = {
  id: number;
}

export async function getGame(request: GetGameRequest) {
  return api.get(`admin/games/${request.id}`).json<{ game: Game }>();
}

export type UpdateGameBody = {
  title?: string;
  sketch?: string | null;
  description?: string | null;
  enabled?: boolean;
  public?: boolean;
  writeup_required?: boolean;
  member_limit_min?: number;
  member_limit_max?: number;
  started_at?: number;
  frozen_at?: number;
  ended_at?: number;
}

export type UpdateGameRequest = UpdateGameBody & {
  id: number;
}

export async function updateGame(request: UpdateGameRequest) {
  const { id, ...body } = request;
  return api.put(`admin/games/${id}`, { json: body }).json<{ game: Game }>();
}

export type DeleteGameRequest = {
  id?: number;
}

export async function deleteGame(request: DeleteGameRequest) {
  return api.delete(`admin/games/${request.id}`).json<Record<string, never>>();
}
