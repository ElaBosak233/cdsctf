package model

// Pod is the minimum unit of Instance operation.
type Pod struct {
	ID          uint        `json:"id"`
	GameID      uint        `json:"game_id"`
	Game        *Game       `json:"game"`
	UserID      uint        `json:"user_id"`
	User        *User       `json:"user"`
	TeamID      uint        `json:"team_id"`
	Team        *Team       `json:"team"`
	ChallengeID uint        `json:"challenge_id"`
	Challenge   *Challenge  `json:"challenge"`
	RemovedAt   int64       `json:"removed_at"`
	Instances   []*Instance `json:"instances"`
}
