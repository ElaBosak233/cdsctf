export type Game = {
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

  icon_hash?: string;
  poster_hash?: string;

  created_at?: number;
};

export type GameMini = {
  id?: number;
  title?: string;
  sketch?: string;
  started_at?: number;
  frozen_at?: number;
  ended_at?: number;
  icon_hash?: string;
  poster_hash?: string;
};

export type ScoreRecord = {
  team?: ScoreboardTeam;
  submissions?: Array<ScoreboardSubmission>;
};

export type ScoreboardTeam = {
  id?: number;
  name?: string;
  slogan?: string;
  avatar_hash?: string;
  pts?: number;
  rank?: number;
};

export type ScoreboardSubmission = {
  id?: number;
  user_id?: number;
  user_name?: string;
  user_avatar_hash?: string;
  challenge_id?: number;
  challenge_title?: string;
  pts?: number;
  created_at?: number;
};
