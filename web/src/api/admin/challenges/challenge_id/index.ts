import { Challenge, Env } from "@/models/challenge";
import { WebResponse } from "@/types";
import { alova } from "@/utils/alova";

export interface UpdateChallengeRequest {
    id?: string | null;
    title?: string | null;
    tags?: Array<string> | null;
    description?: string | null;
    category?: number | null;
    has_attachment?: boolean | null;
    is_public?: boolean | null;
    is_dynamic?: boolean | null;
}

export async function updateChallenge(request: UpdateChallengeRequest) {
    return alova.Put<WebResponse<Challenge>>(
        `/admin/challenges/${request?.id}`,
        request,
        {
            cacheFor: 0,
        }
    );
}

export interface DeleteChallengeRequest {
    id?: string;
}

export async function deleteChallenge(request: DeleteChallengeRequest) {
    return alova.Delete<WebResponse<never>>(`/admin/challenges/${request.id}`);
}
