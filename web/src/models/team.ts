export interface Team {
    id?: number;
    game_id?: number;
    name?: string;
    email?: string;
    slogan?: string;
    state?: State;

    pts?: number;
    rank?: number;
}

export enum State {
    Banned = 0,
    Preparing = 1,
    Pending = 2,
    Passed = 3,
}
