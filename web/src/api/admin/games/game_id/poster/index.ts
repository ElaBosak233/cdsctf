import { api } from "@/utils/query";

type DeleteGamePosterRequest = {
  game_id: number;
};

export async function deleteGamePoster(request: DeleteGamePosterRequest) {
  return api
    .delete(`games/${request.game_id}/poster`)
    .json<Record<string, never>>();
}
