import { Challenge } from "@/models/challenge";
import { WebResponse } from "@/types";
import { alova } from "@/utils/alova";

export interface GetChallengeRequest {
    id?: string;
}

export async function getChallenge(request: GetChallengeRequest) {
    return alova.Get<WebResponse<Challenge>>(`/challenges/${request.id}`);
}
