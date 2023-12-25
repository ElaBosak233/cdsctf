package data

// GameEvent 比赛事件
type GameEvent struct {
	GameEventId int64  `xorm:"'id' pk autoincr" json:"id"`
	GameId      int64  `xorm:"'game_id' index notnull" json:"game_id"`                 // 比赛 Id
	UserId      string `xorm:"'user_id' varchar(36) index notnull" json:"user_id"`     // 用户 Id
	ChallengeId string `xorm:"'challenge_id' varchar(36) notnull" json:"challenge_id"` // 题目 Id
	TeamId      string `xorm:"'team_id' index notnull" json:"team_id"`                 // 团队 Id
	Score       int64  `xorm:"'score' notnull" json:"score"`                           // 得分
}
