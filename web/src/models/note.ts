export interface Note {
  id: number;
  content: string;
  user_id: number;
  user_name?: string;
  user_has_avatar?: boolean;
  challenge_id: number;
  challenge_title?: string;
  challenge_category?: number;
  public: boolean;
  created_at: string;
  updated_at: string;
}
