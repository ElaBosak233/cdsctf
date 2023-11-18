/*
比赛
*/

package data

import "time"

type Game struct {
	Id           string    `xorm:"pk unique 'id' notnull"`
	Title        string    `xorm:"varchar(50) 'title' notnull"`
	TeamIds      []string  `xorm:"json 'team_ids'"`
	ChallengeIds []string  `xorm:"json 'challenge_ids'"`
	CreatedAt    time.Time `xorm:"created 'created_at'"`
	UpdatedAt    time.Time `xorm:"updated 'updated_at'"`
}
