package m2m

type UserGroup struct {
	UserId  string `xorm:"'user_id' varchar(36) index" json:"user_id" binding:"required" msg:"user_id 不能为空"`
	GroupId string `xorm:"'group_id' varchar(36) index" json:"group_id" binding:"required" msg:"group_id 不能为空"`
}
