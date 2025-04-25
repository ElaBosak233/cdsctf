import { Env } from "@/models/challenge";
import { WebResponse } from "@/types";
import { alova } from "@/utils/alova";

export interface StopEnvRequest {
    id: string;
}

export async function stopEnv(request: StopEnvRequest) {
    return alova.Post<WebResponse<unknown>>(
        `/envs/${request.id}/stop`,
        request
    );
}

export interface RenewEnvRequest {
    id: string;
    team_id?: number;
    game_id?: number;
}

export async function renewEnv(request: RenewEnvRequest) {
    return alova.Post<WebResponse<Env>>(`/envs/${request.id}/renew`, request);
}
