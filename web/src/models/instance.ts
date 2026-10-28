import type { Timestamp } from "@/types";
import type { Port } from "./challenge";

export type InstanceState =
  | "starting"
  | "running"
  | "unhealthy"
  | "succeeded"
  | "failed"
  | "stopping"
  | "unknown";

export type InstanceReason =
  | "running"
  | "scheduling"
  | "initializing"
  | "pulling_image"
  | "starting"
  | "not_ready"
  | "image_pull_failed"
  | "invalid_configuration"
  | "crash_loop_back_off"
  | "container_failed"
  | "pod_failed"
  | "completed"
  | "stopping"
  | "invalid_metadata"
  | "unknown";

export type Instance = {
  id: string;
  game_id: number;
  user_id: number;
  team_id: number;
  challenge_id: number;
  nats?: Array<Nat>;
  ports?: Array<Port>;
  public_entry?: string;

  mode: "practice" | "game" | "debug";
  state: InstanceState;
  reason_code: InstanceReason;
  ready: boolean;
  ready_containers: number;
  total_containers: number;
  created_at: Timestamp;
  expires_at: Timestamp | null;
  renewal_count: number | null;
  renewal_limit: number;
  can_renew: boolean;
  can_stop: boolean;

  /** Compatibility fields; use the typed state and lifecycle fields above. */
  status?: string;
  reason?: string;

  renew?: number;
  duration?: number;
  started_at?: Timestamp;
};

export type Nat = {
  port: number;
  node_port: number;
  protocol: "TCP" | "UDP";
};
