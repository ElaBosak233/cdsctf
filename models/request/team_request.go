package request

type TeamCreateRequest struct {
	Name        string `binding:"required" json:"name"`
	Description string `json:"description"`
	CaptainId   int64  `binding:"required" json:"captain_id"`
}

type TeamUpdateRequest struct {
	TeamId      int64  `binding:"required" json:"id"`
	Name        string `binding:"required" json:"name"`
	Description string `json:"description"`
	CaptainId   int64  `binding:"required" json:"captain_id"`
}

type TeamFindRequest struct {
	TeamId    int64  `json:"id"`
	TeamName  string `json:"name"`
	CaptainId int64  `json:"captain_id"`
	Page      int    `json:"page"`
	Size      int    `json:"size"`
}

type TeamBatchFindRequest struct {
	TeamId    []int64 `json:"id"`
	TeamName  string  `json:"name"`
	CaptainId int64   `json:"captain_id"`
	Page      int     `json:"page"`
	Size      int     `json:"size"`
}

type TeamBatchFindByUserIdRequest struct {
	UserId []int64 `json:"user_id"`
}

type TeamDeleteRequest struct {
	TeamId int64 `binding:"required" json:"id"`
}

type TeamJoinRequest struct {
	TeamId int64 `binding:"required" json:"team_id"`
	UserId int64 `binding:"required" json:"user_id"`
}

type TeamQuitRequest struct {
	TeamId int64 `binding:"required" json:"team_id"`
	UserId int64 `binding:"required" json:"user_id"`
}
