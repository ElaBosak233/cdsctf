package data

// GameEvent 比赛事件对象
type GameEvent struct {
	GameEventId int64  `xorm:"'id' pk autoincr" json:"id"`
	GameId      string `xorm:"'game_id' index notnull" json:"game_id"`
	UserId      string `xorm:"'user_id' index notnull" json:"user_id"`
	TeamId      string `xorm:"'team_id' index notnull" json:"team_id"`
	Score       int64  `xorm:"'score' notnull" json:"score"`
	ChallengeId string `xorm:"'challenge_id'" json:"challenge_id"`
}
