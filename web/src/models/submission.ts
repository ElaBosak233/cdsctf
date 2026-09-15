export type SubmissionView = {
  id: number;
  content: string;
  status: Status;
  user_id: number;
  user_name: string;
  user_avatar_hash: string | null;
  challenge_id: number;
  challenge_title: string;
  challenge_category: number;
  team_id: number | null;
  team_name: string | null;
  team_avatar_hash: string | null;
  game_id: number | null;
  game_title: string | null;
  pts: number;
  rank: number;
  created_at: string;
  processing_at: string | null;
  checked_at: string | null;
};

export type SubmissionSummary = Omit<SubmissionView, "content">;

export enum Status {
  Queued = "queued",
  Processing = "processing",
  Correct = "correct",
  Incorrect = "incorrect",
  Cheat = "cheat",
  Expired = "expired",
  Duplicate = "duplicate",
}
