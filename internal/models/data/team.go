/*
团队
*/

package data

type Team struct {
	TeamId    string `xorm:"'id' varchar(36) pk unique notnull" json:"id"`
	TeamName  string `xorm:"varchar(64) 'name' notnull" json:"name"`
	CaptainId string `xorm:"'captain_id' varchar(36) notnull" json:"captain_id"`
	IsLocked  int    `xorm:"'is_locked' int notnull default(0)" json:"is_locked"`
	CreatedAt int64  `xorm:"created 'created_at'" json:"created_at"`
	UpdatedAt int64  `xorm:"updated 'updated_at'" json:"updated_at"`
}
