import { Challenge } from "./challenge";
import { Game } from "./game";

export interface GameChallenge {
	id?: number;
	challenge_id?: number;
	challenge?: Challenge;
	game_id?: number;
	game?: Game;
	is_enabled?: boolean;
	pts?: number;
	max_pts?: number;
	min_pts?: number;
}
