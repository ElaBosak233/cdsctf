package entity

// Pod is the minimum unit of Container operation.
type Pod struct {
	ID          int64 `xorm:"'id' pk autoincr" json:"id"`
	GameID      int64 `xorm:"'game_id'" json:"game_id"`
	UserID      int64 `xorm:"'user_id'" json:"user_id"`
	TeamID      int64 `xorm:"'team_id'" json:"team_id"`
	ChallengeID int64 `xorm:"'challenge_id'" json:"challenge_id"`
	RemovedAt   int64 `xorm:"'removed_at'" json:"removed_at"`

	Containers []Container `xorm:"-" json:"containers"`
}

func (p *Pod) TableName() string {
	return "pod"
}
