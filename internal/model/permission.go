package model

type Permission struct {
	ID   int64  `xorm:"'id' pk autoincr" json:"id"`
	Name string `xorm:"'name' varchar(128) notnull unique"`
}
