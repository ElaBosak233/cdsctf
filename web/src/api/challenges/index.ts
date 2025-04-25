import { Challenge } from "@/models/challenge";
import { Submission } from "@/models/submission";
import { WebResponse } from "@/types";
import { alova } from "@/utils/alova";

export interface GetChallengeRequest {
    id?: string;
    title?: string;
    description?: string;
    category?: number;
    is_dynamic?: boolean;
    page?: number;
    size?: number;
    sorts?: string;
}

export async function getPlaygroundChallenges(request: GetChallengeRequest) {
    return alova.Get<WebResponse<Array<Challenge>>>("/challenges/playground", {
        params: request,
        cacheFor: 0,
    });
}

export interface GetChallengeStatusRequest {
    challenge_ids: Array<string>;
    user_id?: number;
    team_id?: number;
    game_id?: number;
}

export interface ChallengeStatus {
    is_solved?: boolean;
    solved_times?: number;
    pts?: number;
    bloods?: Array<Submission>;
}

export async function getChallengeStatus(request: GetChallengeStatusRequest) {
    return alova.Post<WebResponse<Record<string, ChallengeStatus>>>(
        "/challenges/status",
        request,
        {
            cacheFor: 0,
        }
    );
}
