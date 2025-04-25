import { WebResponse } from "@/types";
import { alova } from "@/utils/alova";

interface DeleteGamePosterRequest {
    game_id: number;
}

export async function deleteGamePoster(request: DeleteGamePosterRequest) {
    return alova.Delete<WebResponse<never>>(
        `/admin/games/${request.game_id}/poster`
    );
}
