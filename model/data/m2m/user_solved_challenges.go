package m2m

type UserSolvedChallenges struct {
	UserId      string `xorm:"index" json:"user_id" binding:"required" msg:"user_id 不能为空"`
	ChallengeId string `xorm:"index" json:"challenge_id" binding:"required" msg:"challenge_id 不能为空"`
}
