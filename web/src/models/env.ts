export interface Env {
    id: string;
    game_id: number;
    // game: Game;
    user_id: number;
    // user: User;
    team_id: number;
    // team: Team;
    challenge_id: string;
    // challenge: Challenge;
    nats?: string;
    ports?: Array<number>;
    public_entry?: string;

    status?: string;
    reason?: string;

    renew?: number;
    duration?: number;
    started_at?: number;
}
