import type { Timestamp } from "@/types";

export type GameNoticeView = {
  id: number;
  game_id: number;
  challenge_id: number | null;
  challenge_title: string | null;
  challenge_category: number | null;
  title: string;
  content: string;
  created_at: Timestamp;
  updated_at: Timestamp;
};

export type AdminGameNoticeView = GameNoticeView & {
  created_by: number | null;
  created_by_name: string | null;
  created_by_avatar_hash: string | null;
  updated_by: number | null;
  updated_by_name: string | null;
  updated_by_avatar_hash: string | null;
};
