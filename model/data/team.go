/*
团队
*/

package data

import "time"

type Team struct {
	Id        string    `xorm:"pk unique 'id' notnull"`
	Name      string    `xorm:"varchar(32) 'name' notnull"`
	CaptainId string    `xorm:"text 'captain_id' notnull" `
	UserIds   []string  `xorm:"json 'user_ids' notnull"`
	CreatedAt time.Time `xorm:"created 'created_at'"`
	UpdatedAt time.Time `xorm:"updated 'updated_at'"`
}
