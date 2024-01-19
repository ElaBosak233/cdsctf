package entity

// User 用户
type User struct {
	UserId    int64  `xorm:"'id' pk autoincr" json:"id"`
	Username  string `xorm:"'username' varchar(36) unique notnull index" json:"username"` // 用户名
	Name      string `xorm:"'name' varchar(36) notnull" json:"name"`                      // 昵称
	Email     string `xorm:"'email' varchar(128) unique notnull" json:"email"`            // 邮箱
	Role      int64  `xorm:"'role'" json:"role"`                                          // 权限等级
	Password  string `xorm:"'password' varchar(255) notnull" json:"password,omitempty"`   // 密码
	CreatedAt int64  `xorm:"'created_at' created" json:"created_at"`                      // 创建时间
	UpdatedAt int64  `xorm:"'updated_at' updated" json:"updated_at"`                      // 更新时间
}

func (u *User) TableName() string {
	return "users"
}
