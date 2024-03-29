package request

type GameTeamCreateRequest struct {
	ID       uint   `json:"-"`
	TeamID   uint   `json:"team_id"`
	UserID   uint   `json:"user_id"`
	Password string `json:"password"`
}

type GameTeamUpdateRequest struct {
	ID        uint  `json:"-"`
	TeamID    uint  `json:"-"`
	IsAllowed *bool `json:"is_allowed"`
}

type GameTeamFindRequest struct {
	GameID uint `json:"game_id" form:"game_id"`
	TeamID uint `json:"team_id" form:"team_id"`
}

type GameTeamDeleteRequest struct {
	GameID uint `json:"game_id"`
	TeamID uint `json:"team_id"`
}
