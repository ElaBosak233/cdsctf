import { Flag } from "./flag";
import { Hint } from "./hint";
import { Port } from "./port";
import { Env } from "./env";
import { Submission } from "./submission";

export interface Challenge {
    id?: number;
    title?: string;
    description?: string;
    category_id?: number;
    has_attachment?: boolean;
    is_practicable?: boolean;
    is_dynamic?: boolean;
    duration?: number;
    image_name?: string;
    memory_limit?: number;
    cpu_limit?: number;
    ports?: Array<Port>;
    envs?: Array<Env>;
    flags?: Array<Flag>;
    hints?: Array<Hint>;
    min_pts?: number;
    max_pts?: number;
}

export interface ChallengeFindRequest {
    id?: number;
    title?: string;
    description?: string;
    category_id?: number;
    is_practicable?: boolean;
    is_dynamic?: boolean;
    is_detailed?: boolean;
    difficulty?: number;
    page?: number;
    size?: number;
    sort_key?: string;
    sort_order?: string;
}

export interface ChallengeUpdateRequest {
    id?: number;
    title?: string;
    description?: string;
    category_id?: number;
    attachment_url?: string;
    is_practicable?: boolean;
    is_dynamic?: boolean;
    difficulty?: number;
    practice_pts?: number;
    duration?: number;
    image_name?: string;
    memory_limit?: number;
    cpu_limit?: number;
    ports?: Array<Port>;
    envs?: Array<Env>;
    flags?: Array<Flag>;
}

export interface ChallengeCreateRequest {
    title?: string;
    description?: string;
    category_id?: number;
    is_practicable?: boolean;
    is_dynamic?: boolean;
    difficulty?: number;
    practice_pts?: number;
    duration?: number;
    image_name?: string;
    memory_limit?: number;
    cpu_limit?: number;
    ports?: Array<Port>;
    envs?: Array<Env>;
}

export interface ChallengeDeleteRequest {
    id?: number;
}

export interface ChallengeStatus {
    is_solved?: boolean;
    solved_times?: number;
    bloods?: Array<Submission>;
}

export interface ChallengeStatusRequest {
    cids: Array<number>;
    user_id?: number;
    team_id?: number;
    game_id?: number;
}
