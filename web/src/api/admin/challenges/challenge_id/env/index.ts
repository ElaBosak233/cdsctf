import { Env } from "@/models/challenge";
import { WebResponse } from "@/types";
import { alova } from "@/utils/alova";

export interface UpdateChallengeEnvRequest {
    id?: string;
    env?: Env;
}

export async function updateChallengeEnv(request: UpdateChallengeEnvRequest) {
    return alova.Put<WebResponse<never>>(
        `/admin/challenges/${request?.id}/env`,
        request,
        {
            cacheFor: 0,
        }
    );
}
