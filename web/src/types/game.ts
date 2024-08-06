import { Status } from "./submission";

export interface Game {
    id?: number;
    title?: string;
    bio?: string;
    description?: string;
    is_enabled?: boolean;
    is_public?: boolean;
    member_limit_min?: number;
    member_limit_max?: number;
    parallel_container_limit?: number;
    is_need_write_up?: boolean;
    started_at?: number;
    ended_at?: number;
    created_at?: number;
    updated_at?: number;
}

export interface GameFindRequest {
    id?: number;
    title?: string;
    is_enabled?: boolean;
    page?: number;
    size?: number;
    sort_key?: string;
    sort_order?: string;
}

export interface GameSubmissionGetRequest {
    id?: number;
    status?: Status;
}

export interface GameChallengeFindRequest {
    game_id?: number;
    is_enabled?: boolean;
}

export interface GameCreateRequest {
    title?: string;
    bio?: string;
    description?: string;
    is_enabled?: boolean;
    is_public?: boolean;
    member_limit_min?: number;
    member_limit_max?: number;
    parallel_container_limit?: number;
    is_need_write_up?: boolean;
    started_at?: number;
    ended_at?: number;
}

export interface GameUpdateRequest {
    id?: number;
    title?: string;
    bio?: string;
    description?: string;
    is_enabled?: boolean;
    is_public?: boolean;
    password?: string;
    member_limit_min?: number;
    member_limit_max?: number;
    parallel_container_limit?: number;
    is_need_write_up?: boolean;
    started_at?: number;
    ended_at?: number;
}

export interface GameDeleteRequest {
    id?: number;
}

export interface GameTeamFindRequest {
    game_id?: number;
    team_id?: number;
}

export interface GameTeamCreateRequest {
    game_id?: number;
    team_id?: number;
}

export interface GameTeamUpdateRequest {
    game_id?: number;
    team_id?: number;
    is_allowed?: boolean;
}

export interface GameTeamDeleteRequest {
    game_id?: number;
    team_id?: number;
}
