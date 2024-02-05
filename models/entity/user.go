package entity

import "time"

type User struct {
	UserID      int64     `xorm:"'id' pk autoincr" json:"id"`                                  // The user's id. As primary key.
	Username    string    `xorm:"'username' varchar(16) unique notnull index" json:"username"` // The user's username. As a unique identifier.
	Nickname    string    `xorm:"'nickname' nvarchar(36) notnull" json:"nickname"`             // The user's nickname. Not unique.
	Description string    `xorm:"'description' text" json:"description"`                       // The user's description.
	Email       string    `xorm:"'email' varchar(64) unique notnull" json:"email"`             // The user's email.
	Role        int64     `xorm:"'role'" json:"role"`                                          // The user's role.
	Password    string    `xorm:"'password' varchar(255) notnull" json:"password,omitempty"`   // The user's password. Crypt.
	CreatedAt   time.Time `xorm:"'created_at' created" json:"created_at"`                      // The user's creation time.
	UpdatedAt   time.Time `xorm:"'updated_at' updated" json:"updated_at"`                      // The user's last update time.
}

func (u *User) TableName() string {
	return "account" // Don't use "user" as table name, because it's a reserved word for PostgreSQL.
}
