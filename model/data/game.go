package data

import "time"

// Game 比赛对象
type Game struct {
	GameId string `xorm:"'id' pk unique notnull"`
	// Title 比赛标题
	Title string `xorm:"varchar(50) 'title' notnull" json:"title"`
	// Description 比赛简介
	Description string `xorm:"text 'description'" json:"description"`
	// NumLimitMin 队伍人数最小值
	NumLimitMin int `xorm:"int 'num_limit_min' notnull default(1)" json:"num_limit_min"`
	// ParallelContainerLimit 并发容器限制
	ParallelContainerLimit int `xorm:"int 'parallel_container_limit' notnull default(2)" json:"parallel_container_limit"`
	// NumLimitMax 队伍人数最大值
	NumLimitMax int `xorm:"int 'num_limit_max' default(99)" json:"num_limit_max"`
	// ChallengeIds 题目 Id
	ChallengeIds []string `xorm:"'challenge_ids'" json:"challenge_ids"`
	// IsNeedWriteUp 是否需要提交 Write Up
	IsNeedWriteUp int `xorm:"'is_need_write_up' int default(0)" json:"is_need_write_up"`
	// StartedAt 比赛开始
	StartedAt time.Time `xorm:"'started_at' notnull" json:"started_at" binding:"required,datetime=2006-01-02 15:04:05"`
	// EndAt 比赛结束
	EndAt time.Time `xorm:"'end_at' notnull" json:"end_at" binding:"required,datetime=2006-01-02 15:04:05"`
	// CreatedAt 比赛创建
	CreatedAt time.Time `xorm:"'created_at' created" json:"created_at"`
}
