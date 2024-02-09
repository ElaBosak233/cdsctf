package request

type PodCreateRequest struct {
	ChallengeID int64 `binding:"required" json:"challenge_id"`
	TeamID      int64 `json:"team_id"`
	GameID      int64 `json:"game_id"`
	UserID      int64 `json:"-"`
}

type PodFindRequest struct {
	IDs         []int64 `json:"id"`
	ChallengeID int64   `json:"challenge_id"`
	UserID      int64   `json:"-"`
	TeamID      int64   `json:"team_id"`
	GameID      int64   `json:"game_id"`
	IsAvailable *bool   `json:"is_available"`
	Page        int     `json:"page"`
	Size        int     `json:"size"`
}

type PodRemoveRequest struct {
	ID     int64 `binding:"required" json:"id"`
	TeamID int64 `json:"team_id"`
	GameID int64 `json:"game_id"`
	UserID int64 `json:"-"`
}

type PodRenewRequest struct {
	ID     int64 `binding:"required" json:"id"`
	TeamID int64 `json:"team_id"`
	GameID int64 `json:"game_id"`
	UserID int64 `json:"-"`
}
