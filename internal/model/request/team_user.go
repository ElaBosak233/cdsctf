package request

type TeamUserCreateRequest struct {
	TeamID      uint   `json:"-"`
	UserID      uint   `json:"user_id"`
	InviteToken string `json:"invite_token"`
}

type TeamUserDeleteRequest struct {
	TeamID uint `binding:"required" json:"team_id"`
	UserID uint `binding:"required" json:"user_id"`
}

type TeamUserJoinRequest struct {
	TeamID      uint   `json:"-"`
	UserID      uint   `json:"-"`
	InviteToken string `json:"invite_token"`
}
