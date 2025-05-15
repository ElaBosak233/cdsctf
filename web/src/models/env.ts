export interface Env {
  id: string;
  game_id: number;
  user_id: number;
  team_id: number;
  challenge_id: string;
  nats?: string;
  ports?: Array<number>;
  public_entry?: string;

  status?: string;
  reason?: string;

  renew?: number;
  duration?: number;
  started_at?: number;
}
