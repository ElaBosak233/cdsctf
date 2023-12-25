package relations

type TeamGame struct {
	TeamId    string `xorm:"'team_id' varchar(36) index" json:"team_id"`
	GameId    int64  `xorm:"'game_id' index" json:"game_id"`
	TeamToken string `xorm:"unique" json:"team_token"`
}
