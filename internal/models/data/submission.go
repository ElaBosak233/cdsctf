package data

type Submission struct {
	SubmissionId int64  `xorm:"'id' pk autoincr" json:"id"`
	Content      string `xorm:"'content' varchar(128) notnull" json:"content"`          // 提交内容
	Status       int    `xorm:"'status' notnull default(0)" json:"status"`              // 评判结果，0 错误，1 正确，2 作弊
	UserId       string `xorm:"'user_id' varchar(36) notnull index" json:"user_id"`     // 用户 Id
	ChallengeId  string `xorm:"'challenge_id' varchar(36) notnull" json:"challenge_id"` // 题目 Id
	TeamId       string `xorm:"'team_id' varchar(36)" json:"team_id"`                   // 团队 Id
	GameId       int64  `xorm:"'game_id' index" json:"game_id"`                         // 比赛 Id
	Pts          int64  `xorm:"'pts' int default(0)" json:"pts"`                        // 提交得分
	CreatedAt    int64  `xorm:"'created_at' created" json:"created_at"`                 // 创建时间
}

/*
有 GameId 一定有 TeamId，没有 TeamId 和 GameId，统计分数的时候用的就是 ChallengeId 对应的 PracticePts，加到 UserId 上
*/
