import { GameChallenge } from "@/models/game_challenge";
import { WebResponse } from "@/types";
import { alova } from "@/utils/alova";

export interface GetGameChallengeRequest {
    game_id?: number;
    challenge_id?: string;
    category?: number;
    is_enabled?: boolean;
    page?: number;
    size?: number;
}

export async function getGameChallenges(request: GetGameChallengeRequest) {
    return alova.Get<WebResponse<Array<GameChallenge>>>(
        `/admin/games/${request.game_id}/challenges`,
        {
            params: request,
        }
    );
}

export interface CreateGameChallengeRequest {
    game_id?: number;
    challenge_id?: string;
    is_enabled?: boolean;
    max_pts?: number;
    min_pts?: number;
    difficulty?: number;
    bonus_ratios?: Array<number>;
}

export async function createGameChallenge(request: CreateGameChallengeRequest) {
    return alova.Post<WebResponse<GameChallenge>>(
        `/admin/games/${request.game_id}/challenges`,
        request
    );
}
