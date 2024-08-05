import { Game } from "@/types/game";
import { Team } from "@/types/team";
import { Challenge } from "@/types/challenge";
import { User } from "@/types/user";

export interface Submission {
    id?: number;
    flag?: string;
    status?: Status;
    user_id?: number;
    user?: User;
    challenge_id?: number;
    challenge?: Challenge;
    team_id?: number;
    team?: Team;
    game_id?: number;
    game?: Game;
    created_at?: number;
    updated_at?: number;
}

export interface GameSubmission extends Submission {
    pts?: number;
    rank?: number;
}

export enum Status {
    Pending = 0,
    Correct = 1,
    Incorrect = 2,
    Cheat = 3,
    Invalid = 4,
}

export interface SubmissionCreateRequest {
    flag?: string;
    challenge_id?: number;
    team_id?: number;
    game_id?: number;
}

export interface SubmissionFindRequest {
    id?: number;
    flag?: string;
    status?: number;
    user_id?: number;
    is_detailed?: boolean;
    challenge_id?: number;
    team_id?: number;
    game_id?: number;
    size?: number;
    page?: number;
}

export interface SubmissionDeleteRequest {
    id?: number;
}
