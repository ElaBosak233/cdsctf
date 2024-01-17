package data

type Instance struct {
	InstanceId  int64  `xorm:"'id' pk autoincr" json:"id"`
	ChallengeId int64  `xorm:"'challenge_id' notnull" json:"challenge_id"`
	Flag        string `xorm:"'flag' varchar(128) notnull" json:"flag"`
	UserId      int64  `xorm:"'user_id' notnull index" json:"user_id"`
	TeamId      int64  `xorm:"'team_id'" json:"team_id"`
	GameId      int64  `xorm:"'game_id' index" json:"game_id"`
	Entry       string `xorm:"'entry' varchar(128) notnull" json:"entry"`
	RemovedAt   int64  `xorm:"'removed_at'" json:"removed_at"`
	CreatedAt   int64  `xorm:"'created_at' created" json:"created_at"`
}

func (i *Instance) TableName() string {
	return "instances"
}
