package data

import "time"

// Game 比赛对象
type Game struct {
	Id    string `xorm:"pk unique 'id' notnull"`
	Title string `xorm:"varchar(50) 'title' notnull"`
	// NumLimitMin 队伍人数最小值
	NumLimitMin int `xorm:"int 'num_limit_min'"`
	// NumLimitMax 队伍人数最大值
	NumLimitMax int `xorm:"int 'num_limit_max'"`
	// TeamIds 参赛队伍
	TeamIds      []string  `xorm:"json 'team_ids'"`
	ChallengeIds []string  `xorm:"json 'challenge_ids'"`
	CreatedAt    time.Time `xorm:"created 'created_at'"`
	UpdatedAt    time.Time `xorm:"updated 'updated_at'"`
}
