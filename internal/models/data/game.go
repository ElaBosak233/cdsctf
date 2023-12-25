package data

// Game 比赛对象
type Game struct {
	GameId                 string   `xorm:"'id' pk unique notnull"`
	Title                  string   `xorm:"varchar(50) 'title' notnull" json:"title"`
	Description            string   `xorm:"text 'description'" json:"description"`
	NumLimitMin            int      `xorm:"int 'num_limit_min' notnull default(1)" json:"num_limit_min"`
	ParallelContainerLimit int      `xorm:"int 'parallel_container_limit' notnull default(2)" json:"parallel_container_limit"`
	NumLimitMax            int      `xorm:"int 'num_limit_max' default(99)" json:"num_limit_max"`
	ChallengeIds           []string `xorm:"'challenge_ids'" json:"challenge_ids"`
	IsNeedWriteUp          bool     `xorm:"'is_need_write_up' bool" json:"is_need_write_up"`
	StartedAt              int64    `xorm:"'started_at' notnull" json:"started_at"`
	EndAt                  int64    `xorm:"'end_at' notnull" json:"end_at"`
	CreatedAt              int64    `xorm:"'created_at' created" json:"created_at"`
}
