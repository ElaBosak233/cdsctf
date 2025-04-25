import { Challenge } from "@/models/challenge";
import { WebResponse } from "@/types";
import { alova } from "@/utils/alova";

export interface GetChallengeRequest {
    id?: string;
    title?: string;
    description?: string;
    category?: number;
    is_public?: boolean;
    is_dynamic?: boolean;
    page?: number;
    size?: number;
    sorts?: string;
}

export async function getChallenges(request: GetChallengeRequest) {
    return alova.Get<WebResponse<Array<Challenge>>>("/admin/challenges", {
        params: request,
        cacheFor: 0,
    });
}

export interface CreateChallengeRequest {
    title?: string;
    description?: string;
    category?: number;
    is_public?: boolean;
    is_dynamic?: boolean;
    has_attachment?: boolean;
}

export async function createChallenge(request: CreateChallengeRequest) {
    return alova.Post<WebResponse<Challenge>>("/admin/challenges", request, {
        cacheFor: 0,
    });
}
