package data

type Instance struct {
	InstanceId  string `xorm:"'id' varchar(36) pk notnull" json:"id"`
	ChallengeId string `xorm:"'challenge_id' varchar(36) notnull" json:"challenge_id"`
	Flag        string `xorm:"'flag' varchar(128) notnull" json:"flag"`
	UserId      string `xorm:"'user_id' varchar(36) notnull index" json:"user_id"`
	TeamId      string `xorm:"'team_id' varchar(36)" json:"team_id"`
	GameId      int64  `xorm:"'game_id' index" json:"game_id"`
	Entry       string `xorm:"'entry' varchar(128) notnull" json:"entry"`
	RemovedAt   int64  `xorm:"'removed_at'" json:"removed_at"`
	CreatedAt   int64  `xorm:"'created_at' created" json:"created_at"`
}
