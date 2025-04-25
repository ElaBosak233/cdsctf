import { User } from "./user";

export interface Team {
    id?: number;
    game_id?: number;
    name?: string;
    email?: string;
    slogan?: string;
    state?: State;

    pts?: number;
    rank?: number;

    deleted_at?: number;

    users?: Array<User>;
}

export enum State {
    Banned = 0,
    Preparing = 1,
    Pending = 2,
    Passed = 3,
}
