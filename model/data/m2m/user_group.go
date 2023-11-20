package m2m

type UserGroup struct {
	UserId  string `xorm:"index" json:"user_id" binding:"required" msg:"user_id 不能为空"`
	GroupId string `xorm:"index" json:"group_id" binding:"required" msg:"group_id 不能为空"`
}
