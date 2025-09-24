export interface GameChallenge {
  game_id?: number;
  challenge_id?: number;
  challenge_title?: string;
  challenge_category?: number;
  is_enabled?: boolean;
  difficulty?: number;
  max_pts?: number;
  min_pts?: number;
  bonus_ratios?: Array<number>;
  frozen_at?: number;
  pts?: number;
}
