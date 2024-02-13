package model

type GameChallenge struct {
	GameChallengeRelationId int64 `xorm:"'id' pk autoincr" json:"id"`
	GameId                  int64 `xorm:"'game_id' index unique(challenge_game_idx)" json:"game_id"`
	ChallengeId             int64 `xorm:"'challenge_id' index unique(challenge_game_idx)" json:"challenge_id"`
	IsEnabled               *bool `xorm:"'is_enabled' bool default(false) notnull" json:"is_enabled"`
	MaxPts                  int64 `xorm:"'max_pts' int default(1000) notnull" json:"max_pts"`
	MinPts                  int64 `xorm:"'min_pts' int default(200) notnull" json:"min_pts"`
}

func (c *GameChallenge) TableName() string {
	return "game_challenge"
}
