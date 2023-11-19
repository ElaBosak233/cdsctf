package data

import "time"

type Submission struct {
	Answer    string    `xorm:"varchar(128) 'answer' notnull" json:"answer"`
	UserId    string    `xorm:"text 'user_id' notnull" json:"user_id"`
	TeamId    string    `xorm:"text 'team_id' notnull" json:"team_id"`
	CreatedAt time.Time `xorm:"created 'created_at'" json:"created_at"`
	UpdatedAt time.Time `xorm:"updated 'updated_at'" json:"updated_at"`
}
