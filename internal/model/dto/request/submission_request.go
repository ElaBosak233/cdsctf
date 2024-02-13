package request

type SubmissionCreateRequest struct {
	Flag        string `json:"flag" binding:"required"`         // 提交内容
	UserID      int64  `json:"-"`                               // 用户 Id
	ChallengeID int64  `json:"challenge_id" binding:"required"` // 题目 Id
	TeamID      int64  `json:"team_id"`                         // 团队 Id
	GameID      int64  `json:"game_id"`                         // 比赛 Id
}

type SubmissionDeleteRequest struct {
	SubmissionId int64 `json:"id" binding:"required"`
}

type SubmissionFindRequest struct {
	UserID      int64    `json:"user_id"`      // 用户 Id
	Status      int      `json:"status"`       // 评判结果
	ChallengeID int64    `json:"challenge_id"` // 题目 Id
	TeamID      int64    `json:"team_id"`      // 团队 Id
	GameID      int64    `json:"game_id"`      // 比赛 Id
	IsDetailed  int      `json:"is_detailed"`  // 是否详细
	SortBy      []string `json:"sort_by"`      // 排序参数
	Page        int      `json:"page"`         // 页码
	Size        int      `json:"size"`         // 每页大小
}

type SubmissionBatchFindRequest struct {
	UserID           int64    `json:"user_id"`            // 用户 Id
	Status           int      `json:"status"`             // 评估结果
	ChallengeID      []int64  `json:"challenge_id"`       // 题目 Id 数组
	SizePerChallenge int      `json:"size_per_challenge"` // 每道题查询量
	TeamID           int64    `json:"team_id"`            // 团队 Id
	GameID           int64    `json:"game_id"`            // 比赛 Id
	IsDetailed       bool     `json:"is_detailed"`        // 是否详细
	SortBy           []string `json:"sort_by"`            // 排序参数
	Size             int      `json:"size"`               // 每页大小
}
