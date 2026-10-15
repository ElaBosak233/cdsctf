import type { UserSummary } from "@/models/user";

export type TeamView = {
  id: number;
  game_id: number;
  name: string;
  email: string | null;
  slogan: string | null;
  avatar_hash: string | null;
  has_writeup: boolean;
  state: State;
  pts: number;
  rank: number;
  members?: UserSummary[];
};

export type PlayerTeamView = Omit<TeamView, "pts" | "rank"> & {
  pts?: number;
  rank?: number;
};

export enum State {
  Banned = 0,
  Preparing = 1,
  Pending = 2,
  Passed = 3,
}
