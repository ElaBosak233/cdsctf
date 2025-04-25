import { Game } from "./game";
import { Team } from "./team";
import { Challenge } from "./challenge";
import { User } from "./user";

export interface Submission {
    id?: number;
    content?: string;
    status?: Status;
    user_id?: number;
    user?: User;
    challenge_id?: string;
    challenge?: Challenge;
    team_id?: number;
    team?: Team;
    game_id?: number;
    game?: Game;
    pts?: number;
    rank?: number;
    created_at?: number;
}

export enum Status {
    Pending = 0,
    Correct = 1,
    Incorrect = 2,
    Cheat = 3,
    Expired = 4,
    Duplicate = 5,
}
