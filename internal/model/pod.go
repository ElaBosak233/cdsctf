package model

// Pod is the minimum unit of Container operation.
type Pod struct {
	ID          uint       `json:"id"`
	GameID      uint       `json:"game_id"`
	Game        *Game      `json:"game,omitempty"`
	UserID      uint       `json:"user_id"`
	User        *User      `json:"user,omitempty"`
	TeamID      uint       `json:"team_id"`
	Team        *Team      `json:"team,omitempty"`
	ChallengeID uint       `json:"challenge_id"`
	Challenge   *Challenge `json:"challenge,omitempty"`
	ContainerID uint       `json:"container_id"`
	Container   *Container `json:"container,omitempty"`
	RemovedAt   int64      `json:"removed_at"`
}
