import { Env } from "@/models/env";
import { alova } from "@/utils/alova";
import { WebResponse } from "@/types";

export interface GetEnvRequest {
    id?: string;
    game_id?: number;
    user_id?: number;
    team_id?: number;
    challenge_id?: string;
}

export async function getEnvs(request: GetEnvRequest) {
    return alova.Get<WebResponse<Array<Env>>>("/envs", {
        params: request,
    });
}

export interface CreateEnvRequest {
    game_id?: number;
    team_id?: number;
    challenge_id?: string;
}

export async function createEnv(request: CreateEnvRequest) {
    return alova.Post<WebResponse<Env>>("/envs", request, {
        timeout: 0,
    });
}
