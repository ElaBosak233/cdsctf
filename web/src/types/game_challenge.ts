import { Challenge } from "./challenge";
import { Game } from "./game";

export interface GameChallenge {
    challenge_id?: number;
    challenge?: Challenge;
    game_id?: number;
    game?: Game;
    is_enabled?: boolean;
    difficulty?: number;
    max_pts?: number;
    min_pts?: number;
    first_blood_reward_ratio?: number;
    second_blood_reward_ratio?: number;
    third_blood_reward_ratio?: number;
}

export interface GameChallengeUpdateRequest {
    challenge_id?: number;
    game_id?: number;
    is_enabled?: boolean;
    max_pts?: number;
    min_pts?: number;
    difficulty?: number;
    first_blood_reward_ratio?: number;
    second_blood_reward_ratio?: number;
    third_blood_reward_ratio?: number;
}

export interface GameChallengeDeleteRequest {
    challenge_id?: number;
    game_id?: number;
}

export interface GameChallengeCreateRequest {
    game_id?: number;
    challenge_id?: number;
    is_enabled?: boolean;
    max_pts?: number;
    min_pts?: number;
    difficulty?: number;
    first_blood_reward_ratio?: number;
    second_blood_reward_ratio?: number;
    third_blood_reward_ratio?: number;
}
