import { Challenge } from "./challenge";
import { Game } from "./game";

export interface GameChallenge {
    challenge_id?: string;
    challenge?: Challenge;
    game_id?: number;
    game?: Game;
    is_enabled?: boolean;
    difficulty?: number;
    max_pts?: number;
    min_pts?: number;
    bonus_ratios?: Array<number>;
    frozen_at?: number;
    pts?: number;
}
