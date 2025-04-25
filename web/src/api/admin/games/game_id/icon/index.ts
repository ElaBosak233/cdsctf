import { WebResponse } from "@/types";
import { alova } from "@/utils/alova";

interface DeleteGameIconRequest {
    game_id: number;
}

export async function deleteGameIcon(request: DeleteGameIconRequest) {
    return alova.Delete<WebResponse<never>>(
        `/admin/games/${request.game_id}/icon`
    );
}
