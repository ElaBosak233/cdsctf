import type { Submission } from "./submission";
import type { Team } from "./team";

export interface Game {
  id?: number;
  title?: string;
  sketch?: string;
  description?: string;
  enabled?: boolean;
  public?: boolean;
  writeup_required?: boolean;

  member_limit_min?: number;
  member_limit_max?: number;

  started_at?: number;
  frozen_at?: number;
  ended_at?: number;

  has_icon?: boolean;
  has_poster?: boolean;

  created_at?: number;
}

export interface GameMini {
  id?: number;
  title?: string;
  sketch?: string;
  started_at?: number;
  frozen_at?: number;
  ended_at?: number;
  has_icon?: boolean;
  has_poster?: boolean;
}

export interface ScoreRecord {
  team?: Team;
  submissions?: Array<Submission>;
}
