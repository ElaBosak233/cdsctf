export type GameChallengeView = {
  game_id: number;
  challenge_id: number;
  challenge_title: string;
  challenge_category: number;
  enabled: boolean;
  difficulty: number;
  max_pts: number;
  min_pts: number;
  bonus_ratios: number[];
  frozen_at: number | null;
  pts: number;
};

export type GameChallengeSummary = {
  game_id: number;
  challenge_id: number;
  challenge_title: string;
  challenge_category: number;
  pts: number;
  frozen_at: number | null;
};
