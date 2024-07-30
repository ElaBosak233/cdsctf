import { Challenge } from "./challenge";
import { Game } from "./game";
import { Team } from "./team";
import { User } from "./user";

export interface Notice {
    id?: number;
    type?: string;
    content?: string;
    game_id?: number;
    game?: Game;
    team_id?: number;
    team?: Team;
    user_id?: number;
    user?: User;
    challenge_id?: number;
    challenge?: Challenge;
    created_at?: number;
    updated_at?: number;
}

export interface NoticeCreateRequest {
    type?: string;
    content?: string;
    game_id?: number;
    team_id?: number;
    user_id?: number;
    challenge_id?: number;
}

export interface NoticeUpdateRequest {
    id?: number;
    type?: string;
    content?: string;
    game_id?: number;
    team_id?: number;
    user_id?: number;
    challenge_id?: number;
}

export interface NoticeDeleteRequest {
    id?: number;
    game_id?: number;
}

export interface NoticeFindRequest {
    id?: number;
    type?: string;
    game_id?: number;
}
