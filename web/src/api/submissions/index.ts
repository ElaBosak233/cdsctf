import { Submission } from "@/models/submission";
import { WebResponse } from "@/types";
import { alova } from "@/utils/alova";

export interface CreateSubmissionRequest {
    content?: string;
    challenge_id?: string;
    team_id?: number;
    game_id?: number;
}

export async function createSubmission(request: CreateSubmissionRequest) {
    return alova.Post<WebResponse<Submission>>("/submissions", request);
}

export interface GetSubmissionRequest {
    id?: number;
    content?: string;
    status?: number;
    user_id?: number;
    is_detailed?: boolean;
    challenge_id?: string;
    team_id?: number;
    game_id?: number;
    size?: number;
    page?: number;
    sorts?: string;

    is_desensitized?: boolean;
}

export async function getSubmission(request: GetSubmissionRequest) {
    return alova.Get<WebResponse<Array<Submission>>>("/submissions", {
        params: request,
        cacheFor: 0,
    });
}
