package relations

type ChallengeGame struct {
	ChallengeId string `xorm:"'challenge_id' index varchar(36)" json:"challenge_id"`
	GameId      string `xorm:"'game_id' index varchar(36)" json:"game_id"`
	IsEnabled   bool   `xorm:"'is_enabled' bool default(0) notnull" json:"is_enabled"` // 是否启用
	MaxPts      int64  `xorm:"'max_pts' int default(1000) notnull" json:"max_pts"`     // 最大得分
	MinPts      int64  `xorm:"'min_pts' int default(200) notnull" json:"min_pts"`      // 最小得分
}
