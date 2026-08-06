export type GameDetail = {
  id: number;
  title: string;
  sketch: string | null;
  description: string | null;
  enabled: boolean;
  public: boolean;
  writeup_required: boolean;
  member_limit_min: number;
  member_limit_max: number;
  timeslots: Timeslot[];
  started_at: number;
  frozen_at: number;
  ended_at: number;
  icon_hash: string | null;
  poster_hash: string | null;
  created_at: number;
};

export type GameView = Pick<
  GameDetail,
  | "id"
  | "title"
  | "sketch"
  | "description"
  | "writeup_required"
  | "started_at"
  | "frozen_at"
  | "ended_at"
  | "icon_hash"
  | "poster_hash"
>;

export type Timeslot = {
  label: string;
  started_at: number;
  ended_at: number;
};

export type GameSummary = {
  id: number;
  title: string;
  sketch: string | null;
  started_at: number;
  frozen_at: number;
  ended_at: number;
  icon_hash: string | null;
  poster_hash: string | null;
};

export type ScoreboardEntry = {
  team: ScoreboardTeam;
  submissions: ScoreboardSubmission[];
};

export type ScoreboardTeam = {
  id: number;
  name: string;
  slogan: string | null;
  avatar_hash: string | null;
  pts: number;
  rank: number;
};

export type ScoreboardSubmission = {
  id: number;
  user_id: number;
  user_name: string;
  user_avatar_hash: string | null;
  challenge_id: number;
  challenge_title: string;
  pts: number;
  created_at: number;
};
