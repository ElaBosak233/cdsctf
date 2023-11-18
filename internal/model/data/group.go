/*
用户组
*/

package data

import "time"

type Group struct {
	Id          string    `xorm:"pk unique 'id' notnull"`
	Name        string    `xorm:"unique notnull"`
	Permissions []string  `xorm:"json 'permissions'"`
	UserIds     []string  `xorm:"json 'user_ids'"`
	CreatedAt   time.Time `xorm:"created 'created_at'"`
	UpdatedAt   time.Time `xorm:"updated 'updated_at'"`
}
