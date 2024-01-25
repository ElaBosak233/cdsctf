package entity

import "time"

type Team struct {
	TeamId      int64     `xorm:"'id' pk autoincr" json:"id"`
	Name        string    `xorm:"'name' nvarchar(36) notnull" json:"name"`                       // 团队名
	Description string    `xorm:"'description' text" json:"description"`                         // 团队简介
	CaptainId   int64     `xorm:"'captain_id' notnull" json:"captain_id,omitempty"`              // 队长用户 Id
	IsLocked    bool      `xorm:"'is_locked' notnull default(false)" json:"is_locked,omitempty"` // 是否锁定
	CreatedAt   time.Time `xorm:"'created_at' created" json:"created_at,omitempty"`              // 创建时间
	UpdatedAt   time.Time `xorm:"'updated_at' updated" json:"updated_at,omitempty"`              // 更新时间
}

func (t *Team) TableName() string {
	return "teams"
}
