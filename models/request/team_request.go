package request

type TeamCreateRequest struct {
	Name        string `binding:"required" json:"name"`
	Description string `json:"description"`
	CaptainId   int64  `binding:"required" json:"captain_id"`
}

type TeamUpdateRequest struct {
	ID          int64  `binding:"required" json:"id"`
	Name        string `json:"name"`
	Description string `json:"description"`
	CaptainId   int64  `json:"captain_id"`
	IsLocked    *bool  `json:"is_locked"`
}

type TeamFindRequest struct {
	ID        int64  `json:"id"`
	TeamName  string `json:"name"`
	CaptainId int64  `json:"captain_id"`
	Page      int    `json:"page"`
	Size      int    `json:"size"`
}

type TeamBatchFindRequest struct {
	ID        []int64 `json:"id"`
	TeamName  string  `json:"name"`
	CaptainID int64   `json:"captain_id"`
	Page      int     `json:"page"`
	Size      int     `json:"size"`
}

type TeamBatchFindByUserIdRequest struct {
	UserID []int64 `json:"user_id"`
}

type TeamDeleteRequest struct {
	ID int64 `binding:"required" json:"id"`
}

type TeamJoinRequest struct {
	TeamID int64 `binding:"required" json:"team_id"`
	UserID int64 `binding:"required" json:"user_id"`
}

type TeamQuitRequest struct {
	TeamID int64 `binding:"required" json:"team_id"`
	UserID int64 `binding:"required" json:"user_id"`
}
