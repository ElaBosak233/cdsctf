package request

type TeamCreateRequest struct {
	Name        string `binding:"required" json:"name"`
	Description string `json:"description"`
	CaptainId   uint   `binding:"required" json:"captain_id"`
}

type TeamUpdateRequest struct {
	ID          uint   `json:"-"`
	Name        string `json:"name"`
	Description string `json:"description"`
	CaptainId   uint   `json:"captain_id"`
	IsLocked    *bool  `json:"is_locked"`
}

type TeamFindRequest struct {
	ID        uint   `json:"id"`
	Name      string `json:"name"`
	CaptainID uint   `json:"captain_id"`
	GameID    *uint  `json:"game_id"`
	Page      int    `json:"page"`
	Size      int    `json:"size"`
}

type TeamBatchFindRequest struct {
	ID        []int64 `json:"id"`
	TeamName  string  `json:"name"`
	CaptainID uint    `json:"captain_id"`
	Page      int     `json:"page"`
	Size      int     `json:"size"`
}

type TeamBatchFindByUserIdRequest struct {
	UserID []uint `json:"user_id"`
}

type TeamDeleteRequest struct {
	ID uint `json:"-"`
}

type TeamJoinRequest struct {
	TeamID uint `binding:"required" json:"team_id"`
	UserID uint `binding:"required" json:"user_id"`
}

type TeamQuitRequest struct {
	TeamID uint `binding:"required" json:"team_id"`
	UserID uint `binding:"required" json:"user_id"`
}
