package request

type InstanceCreateRequest struct {
	ChallengeId string `binding:"required" json:"challenge_id"`
	UserId      string `json:"-"`
}

type InstanceFindRequest struct {
	ChallengeId string `json:"challenge_id"`
	UserId      string `json:"-"`
	TeamId      string `json:"team_id"`
	GameId      int64  `json:"game_id"`
	IsAvailable int    `json:"is_available"`
	Page        int    `json:"page"`
	Size        int    `json:"size"`
}

type InstanceRemoveRequest struct {
	InstanceId string `binding:"required" json:"id"`
}

type InstanceRenewRequest struct {
	InstanceId string `binding:"required" json:"id"`
}
