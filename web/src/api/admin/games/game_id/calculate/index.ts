import { api } from "@/utils/query";

export type CalculateGameRequest = {
  game_id: number;
};

export async function calculateGame(request: CalculateGameRequest) {
  return api
    .post(`admin/games/${request.game_id}/calculate`)
    .json<Record<string, never>>();
}
