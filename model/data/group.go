package data

import "time"

// Group 用户组对象
type Group struct {
	Id          string    `xorm:"pk unique 'id' notnull" json:"id"`
	Name        string    `xorm:"'name' varchar(32) unique notnull" json:"name" binding:"required,min=3,max=32" msg:"用户组名长度不能小于3大于32"`
	Permissions []string  `xorm:"varchar(255) 'permissions'" json:"permissions"`
	CreatedAt   time.Time `xorm:"created 'created_at'" json:"created_at"`
	UpdatedAt   time.Time `xorm:"updated 'updated_at'" json:"updated_at"`
}
