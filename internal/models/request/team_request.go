package request

type TeamCreateRequest struct {
	TeamName  string `binding:"required" json:"name"`
	CaptainId string `binding:"required" json:"captain_id"`
}

type TeamUpdateRequest struct {
	TeamId    string `binding:"required" json:"id"`
	TeamName  string `binding:"required" json:"name"`
	CaptainId string `binding:"required" json:"captain_id"`
}

type TeamFindRequest struct {
	TeamName  string `json:"name"`
	CaptainId string `json:"captain_id"`
	Page      int    `json:"page"`
	Size      int    `json:"size"`
}

type TeamDeleteRequest struct {
	TeamId string `binding:"required" json:"id"`
}

type TeamJoinRequest struct {
	TeamId string `binding:"required" json:"team_id"`
	UserId string `binding:"required" json:"user_id"`
}

type TeamQuitRequest struct {
	TeamId string `binding:"required" json:"team_id"`
	UserId string `binding:"required" json:"user_id"`
}
