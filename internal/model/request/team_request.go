package request

type TeamCreateRequest struct {
	Name        string `binding:"required" json:"name"`
	Description string `json:"description"`
	Email       string `json:"email"`
	CaptainId   uint   `binding:"required" json:"captain_id"`
}

type TeamUpdateRequest struct {
	ID          uint   `json:"-"`
	Name        string `json:"name"`
	Description string `json:"description"`
	Email       string `json:"email"`
	CaptainId   uint   `json:"captain_id"`
	IsLocked    *bool  `json:"is_locked"`
}

type TeamFindRequest struct {
	ID        uint   `json:"id" form:"id"`
	Name      string `json:"name" form:"name"`
	UserID    *uint  `json:"user_id" form:"user_id"`
	CaptainID uint   `json:"captain_id" form:"captain_id"`
	GameID    *uint  `json:"game_id" form:"game_id"`
	Page      int    `json:"page" form:"page"`
	Size      int    `json:"size" form:"size"`
	SortKey   string `json:"sort_key" form:"sort_key"`
	SortOrder string `json:"sort_order" form:"sort_order"`
}

type TeamDeleteRequest struct {
	ID uint `json:"-"`
}

type TeamGetInviteTokenRequest struct {
	ID     uint `json:"-"`
	UserID uint `json:"-"`
}

type TeamUpdateInviteTokenRequest struct {
	ID     uint `json:"-"`
	UserID uint `json:"-"`
}
