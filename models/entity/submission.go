package entity

import "time"

type Submission struct {
	SubmissionId int64     `xorm:"'id' index pk autoincr" json:"id"`
	Flag         string    `xorm:"'flag' varchar(128) notnull" json:"flag,omitempty"` // 提交内容
	Status       int       `xorm:"'status' notnull default(0)" json:"status"`         // 评判结果
	UserId       int64     `xorm:"'user_id' notnull index" json:"user_id"`            // 用户 Id
	ChallengeId  int64     `xorm:"'challenge_id' notnull" json:"challenge_id"`        // 题目 Id
	TeamId       int64     `xorm:"'team_id'" json:"team_id,omitempty"`                // 团队 Id
	GameId       int64     `xorm:"'game_id' index" json:"game_id,omitempty"`          // 比赛 Id
	Pts          int64     `xorm:"'pts' default(0)" json:"pts"`                       // 提交得分
	CreatedAt    time.Time `xorm:"'created_at' created" json:"created_at"`            // 创建时间
	UpdatedAt    time.Time `xorm:"'updated_at' updated" json:"updated_at"`            // 更新时间
}

func (s *Submission) TableName() string {
	return "submission"
}

/*
有 GameId 一定有 TeamId，没有 TeamId 和 GameId，统计分数的时候用的就是 ChallengeId 对应的 PracticePts，加到 UserId 上
0 无意义，1 错误，2 正确，3 作弊，4 重复
*/
