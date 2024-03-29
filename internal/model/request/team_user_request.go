package request

type TeamUserCreateRequest struct {
	TeamID uint `json:"-"`
	UserID uint `json:"user_id"`
}

type TeamUserDeleteRequest struct {
	TeamID uint `binding:"required" json:"team_id"`
	UserID uint `binding:"required" json:"user_id"`
}
