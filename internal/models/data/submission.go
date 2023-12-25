package data

type Submission struct {
	SubmissionId int64  `xorm:"'id' pk autoincr" json:"id"`
	Answer       string `xorm:"varchar(128) 'answer' notnull" json:"answer"`
	AnswerStatus string `xorm:"varchar(32) 'answer_status' notnull" json:"answer_status"`
	UserId       string `xorm:"varchar(32) 'user_id' notnull" json:"user_id"`
	TeamId       string `xorm:"varchar(32) 'team_id' notnull" json:"team_id"`
	GameId       string `xorm:"varchar(32) 'game_id' notnull" json:"game_id"`
	ChallengeId  string `xorm:"text 'challenge_id' notnull" json:"challenge_id"`
	CreatedAt    int64  `xorm:"created 'created_at'" json:"created_at"`
}
