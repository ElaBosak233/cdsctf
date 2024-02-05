package entity

import "time"

type Instance struct {
	InstanceID  int64     `xorm:"'id' pk autoincr" json:"id"`                 // The instance's id. As primary key.
	ChallengeID int64     `xorm:"'challenge_id' notnull" json:"challenge_id"` // The challenge which is related to this instance.
	Flag        string    `xorm:"'flag' varchar(128) notnull" json:"flag"`    // The generated flag which is injected into the instance.
	UserId      int64     `xorm:"'user_id' notnull index" json:"user_id"`     // The user who created this instance.
	TeamId      int64     `xorm:"'team_id'" json:"team_id"`                   // The team which created this instance.
	GameId      int64     `xorm:"'game_id' index" json:"game_id"`             // The game which is related to this instance.
	Entry       string    `xorm:"'entry' varchar(128) notnull" json:"entry"`  // The public entry of this instance.
	RemovedAt   time.Time `xorm:"'removed_at'" json:"removed_at"`             // The time when this instance will be removed.
	CreatedAt   time.Time `xorm:"'created_at' created" json:"created_at"`     // The instance's creation time.
}

func (i *Instance) TableName() string {
	return "instance"
}
