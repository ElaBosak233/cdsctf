package model

type GameChallenge struct {
	ID          uint  `json:"id"`
	GameID      uint  `gorm:"uniqueIndex:game_challenge_idx" json:"game_id"`
	ChallengeID uint  `gorm:"uniqueIndex:game_challenge_idx" json:"challenge_id"`
	IsEnabled   *bool `gorm:"default:false;not null;" json:"is_enabled"`
	MaxPts      int64 `gorm:"default:1000;not null;" json:"max_pts"`
	MinPts      int64 `gorm:"default:200;not null;" json:"min_pts"`
}
