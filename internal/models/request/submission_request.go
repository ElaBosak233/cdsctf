package request

type SubmissionCreateRequest struct {
	Flag        string `json:"flag" binding:"required" msg:"Flag 不能为空"`          // 提交内容
	UserId      int64  `json:"-"`                                                // 用户 Id
	ChallengeId int64  `json:"challenge_id" binding:"required" msg:"题目 Id 不能为空"` // 题目 Id
	TeamId      int64  `json:"team_id"`                                          // 团队 Id
	GameId      int64  `json:"game_id"`                                          // 比赛 Id
}

type SubmissionDeleteRequest struct {
	SubmissionId int64 `json:"id" binding:"required"`
}

type SubmissionFindRequest struct {
	UserId      int64    `json:"user_id"`      // 用户 Id
	Status      int      `json:"status"`       // 评判结果
	ChallengeId int64    `json:"challenge_id"` // 题目 Id
	TeamId      int64    `json:"team_id"`      // 团队 Id
	GameId      int64    `json:"game_id"`      // 比赛 Id
	IsDetailed  int      `json:"is_detailed"`  // 是否详细
	SortBy      []string `json:"sort_by"`      // 排序参数
	Page        int      `json:"page"`         // 页码
	Size        int      `json:"size"`         // 每页大小
}

type SubmissionBatchFindRequest struct {
	UserId      int64   `json:"user_id"`      // 用户 Id
	Status      int     `json:"status"`       // 评估结果
	ChallengeId []int64 `json:"challenge_id"` // 题目 Id 数组
	TeamId      int64   `json:"team_id"`      // 团队 Id
	GameId      int64   `json:"game_id"`      // 比赛 Id
	IsAscend    bool    `json:"is_ascend"`    // 是否升序
	IsDetailed  bool    `json:"is_detailed"`  // 是否详细
	Size        int     `json:"size"`         // 每页大小
}
