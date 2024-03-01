package request

type SubmissionCreateRequest struct {
	Flag        string `json:"flag" binding:"required"`         // 提交内容
	UserID      uint   `json:"-"`                               // 用户 Id
	ChallengeID uint   `json:"challenge_id" binding:"required"` // 题目 Id
	TeamID      *uint  `json:"team_id"`                         // 团队 Id
	GameID      *uint  `json:"game_id"`                         // 比赛 Id
}

type SubmissionDeleteRequest struct {
	SubmissionID uint `json:"id" binding:"required"`
}

type SubmissionFindRequest struct {
	UserID      uint     `json:"user_id"`      // 用户 Id
	Status      int      `json:"status"`       // 评判结果
	ChallengeID uint     `json:"challenge_id"` // 题目 Id
	TeamID      *uint    `json:"team_id"`      // 团队 Id
	GameID      *uint    `json:"game_id"`      // 比赛 Id
	IsDetailed  bool     `json:"is_detailed"`  // 是否详细
	SortBy      []string `json:"sort_by"`      // 排序参数
	Page        int      `json:"page"`         // 页码
	Size        int      `json:"size"`         // 每页大小
}

type SubmissionFindByChallengeIDRequest struct {
	ChallengeID      []uint   `json:"challenge_id"`       // 题目 Id 数组
	UserID           uint     `json:"user_id"`            // 用户 Id
	Status           int      `json:"status"`             // 评估结果
	SizePerChallenge int      `json:"size_per_challenge"` // 每道题查询量
	TeamID           *uint    `json:"team_id"`            // 团队 Id
	GameID           *uint    `json:"game_id"`            // 比赛 Id
	IsDetailed       bool     `json:"is_detailed"`        // 是否详细
	SortBy           []string `json:"sort_by"`            // 排序参数
	Size             int      `json:"size"`               // 每页大小
}
