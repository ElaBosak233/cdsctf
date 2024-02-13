package model

type TeamGame struct {
	TeamGameRelationId int64  `xorm:"'id' pk autoincr" json:"id"`
	TeamId             int64  `xorm:"'team_id' index" json:"team_id"`
	GameId             int64  `xorm:"'game_id' index" json:"game_id"`
	TeamToken          string `xorm:"unique" json:"team_token"`
}
