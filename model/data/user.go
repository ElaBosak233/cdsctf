/*
用户
*/

package data

import "time"

type User struct {
	// Id 自动生成的 uuid
	Id string `xorm:"'id' pk unique notnull" json:"id"`
	// Username 用户名
	Username string `xorm:"'username' varchar(32) unique notnull index" json:"username" binding:"required,min=3,max=32" msg:"用户名长度不能小于3大于32"`
	// Email 邮箱
	Email string `xorm:"'email' text unique notnull index" json:"email" binding:"required,email" msg:"邮箱地址格式不正确"`
	// Password 密码哈希
	Password  string    `xorm:"'password' varchar(255) notnull" json:"password" binding:"required,min=6" msg:"密码不能小于6位"`
	CreatedAt time.Time `xorm:"'created_at' created" json:"created_at"`
	UpdatedAt time.Time `xorm:"'updated_at' updated" json:"updated_at"`
}
