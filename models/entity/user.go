package entity

import "time"

// User 用户
type User struct {
	UserId      int64     `xorm:"'id' pk autoincr" json:"id"`
	Username    string    `xorm:"'username' varchar(16) unique notnull index" json:"username"` // 用户名
	Nickname    string    `xorm:"'nickname' nvarchar(36) notnull" json:"nickname"`             // 昵称
	Description string    `xorm:"'description' text" json:"description"`                       // 简介
	Email       string    `xorm:"'email' varchar(64) unique notnull" json:"email"`             // 邮箱
	Role        int64     `xorm:"'role'" json:"role"`                                          // 权限等级
	Password    string    `xorm:"'password' varchar(255) notnull" json:"password,omitempty"`   // 密码
	CreatedAt   time.Time `xorm:"'created_at' created" json:"created_at"`                      // 创建时间
	UpdatedAt   time.Time `xorm:"'updated_at' updated" json:"updated_at"`                      // 更新时间
}

func (u *User) TableName() string {
	return "user"
}
