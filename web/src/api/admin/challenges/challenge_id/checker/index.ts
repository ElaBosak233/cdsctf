import { WebResponse } from "@/types";
import { alova } from "@/utils/alova";

export interface UpdateChallengeCheckerRequest {
    id?: string;
    checker?: string;
}

export async function updateChallengeChecker(
    request: UpdateChallengeCheckerRequest
) {
    return alova.Put<WebResponse<never>>(
        `/admin/challenges/${request?.id}/checker`,
        request,
        {
            cacheFor: 0,
        }
    );
}
