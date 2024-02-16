package model

import (
	"time"
)

type User struct {
	ID          uint      `json:"id"`                                                                     // The user's id. As primary key.
	Username    string    `gorm:"column:username;type:varchar(16);unique;not null;index" json:"username"` // The user's username. As a unique identifier.
	Nickname    string    `gorm:"column:nickname;type:varchar(36);not null" json:"nickname"`              // The user's nickname. Not unique.
	Description string    `gorm:"column:description;type:text" json:"description"`                        // The user's description.
	Email       string    `gorm:"column:email;varchar(64);unique;not null" json:"email"`                  // The user's email.
	GroupID     uint      `json:"group_id"`                                                               // The user's role.
	Group       *Group    `json:"group"`                                                                  // The user's role.
	Password    string    `gorm:"column:password;type:varchar(255);not null" json:"password,omitempty"`   // The user's password. Crypt.
	CreatedAt   time.Time `json:"created_at"`                                                             // The user's creation time.
	UpdatedAt   time.Time `json:"updated_at"`                                                             // The user's last update time.
	Teams       []*Team   `gorm:"many2many:user_teams;" json:"teams"`                                     // The user's teams.
}
