package relations

type GameChallenge struct {
	GameId      int64 `xorm:"'game_id' index unique(challenge_game_idx)" json:"game_id"`
	ChallengeId int64 `xorm:"'challenge_id' index unique(challenge_game_idx)" json:"challenge_id"`
	IsEnabled   bool  `xorm:"'is_enabled' bool default(false) notnull" json:"is_enabled"` // 是否启用
	MaxPts      int64 `xorm:"'max_pts' int default(1000) notnull" json:"max_pts"`         // 最大得分
	MinPts      int64 `xorm:"'min_pts' int default(200) notnull" json:"min_pts"`          // 最小得分
}

func (c *GameChallenge) TableName() string {
	return "game_challenge"
}
