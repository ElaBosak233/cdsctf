import { Game } from "./game";
import { User } from "./user";
import { Team } from "./team";
import { Challenge } from "./challenge";
import { Nat } from "./nat";

export interface Pod {
    id: number;
    name: string;
    game_id: number;
    game: Game;
    user_id: number;
    user: User;
    team_id: number;
    team: Team;
    challenge_id: number;
    challenge: Challenge;
    removed_at: number;
    nats?: Array<Nat>;
}

export interface PodFindRequest {
    id?: number;
    game_id?: number;
    user_id?: number;
    team_id?: number;
    challenge_id?: number;
    is_available?: boolean;
    page?: number;
    size?: number;
}

export interface PodCreateRequest {
    game_id?: number;
    team_id?: number;
    challenge_id?: number;
}

export interface PodRemoveRequest {
    id: number;
    team_id?: number;
    game_id?: number;
}

export interface PodRenewRequest {
    id: number;
    team_id?: number;
    game_id?: number;
}
