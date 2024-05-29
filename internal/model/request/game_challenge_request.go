package request

type GameChallengeFindRequest struct {
	GameID        uint  `json:"game_id" form:"game_id"`
	ChallengeID   uint  `json:"challenge_id" form:"challenge_id"`
	TeamID        uint  `json:"team_id" form:"team_id"`
	IsEnabled     *bool `json:"is_enabled" form:"is_enabled"`
	SubmissionQty int   `json:"submission_qty" form:"submission_qty"`
}

type GameChallengeCreateRequest struct {
	GameID      uint  `json:"-"`
	ChallengeID uint  `json:"challenge_id"`
	IsEnabled   *bool `json:"is_enabled"`
	MaxPts      int64 `json:"max_pts"`
	MinPts      int64 `json:"min_pts"`
}

type GameChallengeUpdateRequest struct {
	ID          uint  `json:"id"`
	GameID      uint  `json:"-"`
	ChallengeID uint  `json:"challenge_id"`
	IsEnabled   *bool `json:"is_enabled"`
	MaxPts      int64 `json:"max_pts"`
	MinPts      int64 `json:"min_pts"`
}

type GameChallengeDeleteRequest struct {
	ID          uint `json:"-"`
	GameID      uint `json:"-"`
	ChallengeID uint `json:"-"`
}
