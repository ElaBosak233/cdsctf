package request

type PodCreateRequest struct {
	ChallengeID uint  `binding:"required" json:"challenge_id"`
	TeamID      *uint `json:"team_id"`
	GameID      *uint `json:"game_id"`
	UserID      uint  `json:"-"`
}

type PodFindRequest struct {
	ID          uint  `json:"id" form:"id"`
	ChallengeID uint  `json:"challenge_id" form:"challenge_id"`
	UserID      *uint `json:"user_id" form:"user_id"`
	TeamID      *uint `json:"team_id" form:"team_id"`
	GameID      *uint `json:"game_id" form:"game_id"`
	IsAvailable *bool `json:"is_available" form:"is_available"`
	Page        int   `json:"page" form:"page"`
	Size        int   `json:"size" form:"size"`
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
