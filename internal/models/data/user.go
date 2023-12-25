package data

// User 用户
type User struct {
	UserId    string `xorm:"'id' varchar(36) pk unique notnull" json:"id"`
	Username  string `xorm:"'username' varchar(32) unique notnull index" json:"username"`
	Role      int    `xorm:"role int" json:"role"`
	Email     string `xorm:"'email' varchar(128) unique notnull" json:"email"`
	Password  string `xorm:"'password' varchar(255) notnull" json:"password"`
	CreatedAt int64  `xorm:"'created_at' created" json:"created_at"`
	UpdatedAt int64  `xorm:"'updated_at' updated" json:"updated_at"`
}
