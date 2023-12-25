package m2m

type GameChallenge struct {
	GameId      string `xorm:"'game_id' index varchar(36)" json:"game_id" binding:"required" msg:"game_id 不能为空"`
	ChallengeId string `xorm:"'challenge_id' index varchar(36)" json:"challenge_id" binding:"required" msg:"challenge_id 不能为空"`
}
