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
  created_at: number;
};

export type SubmissionSummary = Omit<SubmissionView, "content">;

export enum Status {
  Pending = 0,
  Correct = 1,
  Incorrect = 2,
  Cheat = 3,
  Expired = 4,
  Duplicate = 5,
}
