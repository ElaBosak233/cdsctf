package relations

type UserTeam struct {
	UserId string `xorm:"'user_id' varchar(36) index" json:"user_id" binding:"required" msg:"user_id 不能为空"`
	TeamId string `xorm:"'team_id' varchar(36) index" json:"team_id" binding:"required" msg:"team_id 不能为空"`
}
