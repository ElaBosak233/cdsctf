import { api } from "@/utils/query";

interface DeleteGameIconRequest {
  game_id: number;
}

export async function deleteGameIcon(request: DeleteGameIconRequest) {
  return api
    .delete(`admin/games/${request.game_id}/icon`)
    .json<Record<string, never>>();
}
