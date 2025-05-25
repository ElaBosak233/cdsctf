import { WebResponse } from "@/types";
import { api } from "@/utils/query";

interface DeleteGamePosterRequest {
  game_id: number;
}

export async function deleteGamePoster(request: DeleteGamePosterRequest) {
  return api
    .delete(`admin/games/${request.game_id}/poster`)
    .json<WebResponse<never>>();
}
