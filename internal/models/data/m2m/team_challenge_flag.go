package m2m

type TeamChallengeFlag struct {
	TeamId      string `xorm:"'team_id' varchar(36) index notnull" json:"team_id"`
	ChallengeId string `xorm:"'challenge_id' varchar(36) index notnull" json:"challenge_id"`
	Flag        string `xorm:"'flag' index notnull" json:"flag"`
}
