package m2m

type UserTeam struct {
	UserId string `xorm:"index" json:"user_id" binding:"required" msg:"user_id 不能为空"`
	TeamId string `xorm:"index" json:"team_id" binding:"required" msg:"team_id 不能为空"`
}
