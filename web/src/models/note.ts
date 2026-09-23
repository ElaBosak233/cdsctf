import type { Timestamp } from "@/types";

export type NoteView = {
  id: number;
  content: string;
  user_id: number;
  user_name?: string;
  user_avatar_hash?: string;
  challenge_id: number;
  challenge_title?: string;
  challenge_category?: number;
  public: boolean;
  created_at: Timestamp;
  updated_at: Timestamp;
};
