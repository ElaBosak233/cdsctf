package relations

type GameChallenge struct {
	GameId      string `xorm:"'game_id' index varchar(36)" json:"game_id"`
	ChallengeId string `xorm:"'challenge_id' index varchar(36)" json:"challenge_id"`
}
