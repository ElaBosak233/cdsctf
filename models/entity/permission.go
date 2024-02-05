package entity

type Permission struct {
	PermissionID int64  `xorm:"'id' pk autoincr" json:"id"`
	Name         string `xorm:"'name' varchar(128) notnull unique"`
}
