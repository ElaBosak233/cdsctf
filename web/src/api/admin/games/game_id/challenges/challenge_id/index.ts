import { GameChallenge } from "@/models/game_challenge";
import { WebResponse } from "@/types";
import { alova } from "@/utils/alova";

export interface UpdateGameChallengeRequest {
    game_id?: number;
    challenge_id?: string;
    is_enabled?: boolean;
    max_pts?: number;
    min_pts?: number;
    difficulty?: number;
    bonus_ratios?: Array<number>;
    frozen_at?: number | null;
}

export async function updateGameChallenge(request: UpdateGameChallengeRequest) {
    return alova.Put<WebResponse<GameChallenge>>(
        `/admin/games/${request.game_id}/challenges/${request.challenge_id}`,
        request
    );
}

export interface DeleteGameChallengeRequest {
    challenge_id?: string;
    game_id?: number;
}

export async function deleteGameChallenge(request: DeleteGameChallengeRequest) {
    return alova.Delete<WebResponse<never>>(
        `/admin/games/${request.game_id}/challenges/${request.challenge_id}`,
        request
    );
}
