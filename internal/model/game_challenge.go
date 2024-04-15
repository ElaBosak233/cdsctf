package model

import "gorm.io/gorm"

type GameChallenge struct {
	ID          uint       `json:"id,omitempty"`
	GameID      uint       `gorm:"uniqueIndex:game_challenge_idx" json:"game_id,omitempty"`
	Game        *Game      `json:"game,omitempty"`
	ChallengeID uint       `gorm:"uniqueIndex:game_challenge_idx" json:"challenge_id,omitempty"`
	Challenge   *Challenge `json:"challenge,omitempty"`
	IsEnabled   *bool      `gorm:"default:false;not null;" json:"is_enabled,omitempty"`
	Pts         int64      `gorm:"-" json:"pts,omitempty"`
	MaxPts      int64      `gorm:"default:1000;not null;" json:"max_pts,omitempty"`
	MinPts      int64      `gorm:"default:200;not null;" json:"min_pts,omitempty"`
}

func (g *GameChallenge) BeforeDelete(db *gorm.DB) (err error) {
	db.Table("submissions").Where("game_id = ?", g.GameID).Where("challenge_id = ?", g.ChallengeID).Delete(&Submission{})
	db.Table("notices").Where("game_id = ?", g.GameID).Where("challenge_id = ?", g.ChallengeID).Delete(&Notice{})
	return nil
}
