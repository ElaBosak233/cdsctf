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
	UserID      uint   `json:"user_id" form:"user_id"`           // 用户 Id
	Status      int    `json:"status" form:"status"`             // 评判结果
	ChallengeID uint   `json:"challenge_id" form:"challenge_id"` // 题目 Id
	TeamID      *uint  `json:"team_id" form:"team_id"`           // 团队 Id
	GameID      *uint  `json:"game_id" form:"game_id"`           // 比赛 Id
	IsDetailed  bool   `json:"is_detailed" form:"is_detailed"`   // 是否详细
	Page        int    `json:"page" form:"page"`                 // 页码
	Size        int    `json:"size" form:"size"`                 // 每页大小
	SortKey     string `json:"sort_key" form:"sort_key"`         // 排序参数
	SortOrder   string `json:"sort_order" form:"sort_order"`     // 排序方式
}
