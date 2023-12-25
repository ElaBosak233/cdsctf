package data

type Submission struct {
	SubmissionId int64  `xorm:"'id' pk autoincr" json:"id"`
	Content      string `xorm:"'content' varchar(128) notnull" json:"content"`          // 提交内容
	Status       string `xorm:"'status' varchar(32) notnull" json:"status"`             // 评判结果
	UserId       string `xorm:"'user_id' varchar(36) notnull" json:"user_id"`           // 用户 Id
	ChallengeId  string `xorm:"'challenge_id' varchar(36) notnull" json:"challenge_id"` // 题目 Id
	TeamId       string `xorm:"'team_id' varchar(36)" json:"team_id"`                   // 团队 Id
	GameId       int64  `xorm:"'game_id' index" json:"game_id"`                         // 比赛 Id
	CreatedAt    int64  `xorm:"'created_at' created" json:"created_at"`                 // 创建时间
}
