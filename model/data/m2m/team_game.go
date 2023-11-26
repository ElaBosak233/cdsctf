package m2m

type TeamGame struct {
	TeamId    string `xorm:"'team_id' varchar(36) index" json:"team_id" binding:"required" msg:"team_id 不能为空"`
	GameId    string `xorm:"'game_id' varchar(36) index" json:"game_id" binding:"required" msg:"game_id 不能为空"`
	TeamToken string `xorm:"unique" json:"team_token" binding:"required" msg:"team_token 不能为空"`
}
