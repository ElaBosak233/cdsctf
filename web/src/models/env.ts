import { Port } from "./challenge";

export interface Env {
  id: string;
  game_id: number;
  user_id: number;
  team_id: number;
  challenge_id: string;
  nats?: Array<Nat>;
  ports?: Array<Port>;
  public_entry?: string;

  status?: string;
  reason?: string;

  renew?: number;
  duration?: number;
  started_at?: number;
}

export interface Nat {
  port: number;
  node_port: number;
  protocol: "TCP" | "UDP";
}
