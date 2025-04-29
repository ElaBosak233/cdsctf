export interface Submission {
    id?: number;
    content?: string;
    status?: Status;
    user_id?: number;
    user_name?: String;
    challenge_id?: string;
    challenge_title?: string;
    challenge_category?: number;
    team_id?: number;
    team_name?: string;
    game_id?: number;
    game_title?: string;
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
