package relations

type UserTeam struct {
	UserId int64 `xorm:"'user_id' index" json:"user_id" binding:"required" msg:"user_id 不能为空"`
	TeamId int64 `xorm:"'team_id' index" json:"team_id" binding:"required" msg:"team_id 不能为空"`
}
