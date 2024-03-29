package request

type NoticeFindRequest struct {
	ID     uint   `json:"id" form:"id"`
	GameID uint   `json:"game_id" form:"id"`
	Type   string `json:"type" form:"id"`
}

type NoticeCreateRequest struct {
	GameID      uint   `json:"game_id"`
	ChallengeID *uint  `json:"challenge_id"`
	UserID      *uint  `json:"user_id"`
	TeamID      *uint  `json:"team_id"`
	Type        string `json:"type"`
	Content     string `json:"content"`
}

type NoticeUpdateRequest struct {
	ID          uint   `json:"id"`
	GameID      uint   `json:"game_id"`
	ChallengeID *uint  `json:"challenge_id"`
	UserID      *uint  `json:"user_id"`
	TeamID      *uint  `json:"team_id"`
	Content     string `json:"content"`
}

type NoticeDeleteRequest struct {
	ID uint `json:"id"`
}
