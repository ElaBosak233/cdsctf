/*
团队
*/

package data

import "time"

type Team struct {
	TeamId    string    `xorm:"'id' varchar(36) pk unique notnull" json:"id"`
	Name      string    `xorm:"varchar(64) 'name' notnull" json:"name"`
	CaptainId string    `xorm:"'captain_id' varchar(36) notnull" json:"captain_id"`
	IsLocked  int       `xorm:"'is_locked' int notnull default(0)" json:"is_locked"`
	CreatedAt time.Time `xorm:"created 'created_at'" json:"created_at"`
	UpdatedAt time.Time `xorm:"updated 'updated_at'" json:"updated_at"`
}
