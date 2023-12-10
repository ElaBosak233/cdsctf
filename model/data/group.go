package data

// Group 用户组对象
type Group struct {
	GroupId string `xorm:"'id' varchar(36) pk unique notnull" json:"id"`
	Name    string `xorm:"'name' varchar(32) unique notnull" json:"name"`
}
