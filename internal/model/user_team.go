package model

type UserTeam struct {
	UserTeamRelationId int64 `xorm:"'id' pk autoincr" json:"id"`
	UserId             int64 `xorm:"'user_id' index unique(user_team_idx)" json:"user_id" binding:"required" msg:"user_id 不能为空"`
	TeamId             int64 `xorm:"'team_id' index unique(user_team_idx)" json:"team_id" binding:"required" msg:"team_id 不能为空"`
}

func (u *UserTeam) TableName() string {
	return "user_team"
}
