package request

type SubmissionCreateRequest struct {
	Flag        string `json:"flag"`         // 提交内容
	UserId      string `json:"-"`            // 用户 Id
	ChallengeId string `json:"challenge_id"` // 题目 Id
	TeamId      string `json:"team_id"`      // 团队 Id
	GameId      int64  `json:"game_id"`      // 比赛 Id
}

type SubmissionFindRequest struct {
	UserId      string `json:"user_id"`      // 用户 Id
	Status      int    `json:"status"`       // 评判结果
	ChallengeId string `json:"challenge_id"` // 题目 Id
	TeamId      string `json:"team_id"`      // 团队 Id
	GameId      int64  `json:"game_id"`      // 比赛 Id
	IsDetailed  int    `json:"is_detailed"`  // 是否详细
	Page        int    `json:"page"`         // 页码
	Size        int    `json:"size"`         // 每页大小
}
