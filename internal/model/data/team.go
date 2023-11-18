/*
团队
*/

package data

import "time"

type Team struct {
	Id        string    `xorm:"pk unique 'id' notnull"`
	Name      string    `xorm:"varchar(50) 'name' notnull"`
	UserIds   []string  `xorm:"json 'user_ids'"`
	CreatedAt time.Time `xorm:"created 'created_at'"`
	UpdatedAt time.Time `xorm:"updated 'updated_at'"`
}
