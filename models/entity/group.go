package entity

type Group struct {
	GroupID int64  `xorm:"id pk autoincr" json:"id"`
	Name    string `xorm:"name varchar(36) notnull unique" json:"name"`
}
