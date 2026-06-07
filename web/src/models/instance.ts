import type { Port } from "./challenge";

export type Instance = {
  id: string;
  game_id: number;
  user_id: number;
  team_id: number;
  challenge_id: number;
  nats?: Array<Nat>;
  ports?: Array<Port>;
  public_entry?: string;

  status?: string;
  reason?: string;

  renew?: number;
  duration?: number;
  started_at?: number;
};

export type Nat = {
  port: number;
  node_port: number;
  protocol: "TCP" | "UDP";
};
