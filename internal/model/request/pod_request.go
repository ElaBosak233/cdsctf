package request

type PodCreateRequest struct {
	ChallengeID uint  `binding:"required" json:"challenge_id"`
	TeamID      *uint `json:"team_id"`
	GameID      *uint `json:"game_id"`
	UserID      uint  `json:"-"`
}

type PodFindRequest struct {
	IDs         []uint `json:"id"`
	ChallengeID uint   `json:"challenge_id"`
	UserID      uint   `json:"-"`
	TeamID      *uint  `json:"team_id"`
	GameID      *uint  `json:"game_id"`
	IsAvailable *bool  `json:"is_available"`
	Page        int    `json:"page"`
	Size        int    `json:"size"`
}

type PodRemoveRequest struct {
	ID     uint  `json:"-"`
	TeamID *uint `json:"team_id"`
	GameID *uint `json:"game_id"`
	UserID uint  `json:"-"`
}

type PodRenewRequest struct {
	ID     uint  `json:"-"`
	TeamID *uint `json:"team_id"`
	GameID *uint `json:"game_id"`
	UserID uint  `json:"-"`
}
