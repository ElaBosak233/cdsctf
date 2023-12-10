package data

import "time"

// Challenge 题目
type Challenge struct {
	ChallengeId string `xorm:"'id' varchar(36) pk unique notnull" json:"id"`
	// Title 标题
	Title string `xorm:"varchar(50) 'title' notnull" json:"title"`
	// Description 描述
	Description string `xorm:"text 'description' notnull" json:"description"`
	// AttachmentId 附件 Id
	AttachmentId string `xorm:"json 'attachment_ids'" json:"attachment_id"`
	// IsDynamic 是否为动态题目
	IsDynamic int `xorm:"'is_dynamic' int" json:"is_dynamic" binding:"oneof=0 1"`
	// ExposedPort 内部端口号
	ExposedPort int `xorm:"'exposed_port' int" json:"exposed_port"`
	// Image 镜像名
	Image string `xorm:"text 'image'" json:"image"`
	// Flag 期望的结果
	Flag string `xorm:"text 'flag'" json:"flag"`
	// FlagEnv 环境变量名
	FlagEnv string `xorm:"'flag_env' text" json:"flag_env"`
	// MemoryLimit 内存限制
	MemoryLimit int64 `xorm:"int 'memory_limit'" json:"memory_limit"`
	// Duration 时间限制
	Duration int `xorm:"int 'duration'" json:"duration"`
	// Difficulty 难度
	Difficulty int       `xorm:"int 'difficulty'" json:"difficulty"`
	CreatedAt  time.Time `xorm:"created 'created_at'" json:"created_at"`
	UpdatedAt  time.Time `xorm:"updated 'updated_at'" json:"updated_at"`
}
