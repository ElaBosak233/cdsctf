package model

import (
	"time"
)

type Team struct {
	ID          uint      `json:"id"`                                                // The team's id. As primary key.
	Name        string    `gorm:"type:varchar(36);not null" json:"name"`             // The team's name.
	Description string    `gorm:"type:text" json:"description"`                      // The team's description.
	CaptainID   uint      `gorm:"not null" json:"captain_id,omitempty"`              // The captain's id.
	IsLocked    *bool     `gorm:"not null;default:false" json:"is_locked,omitempty"` // Whether the team is locked. (true/false)
	CreatedAt   time.Time `json:"created_at,omitempty"`                              // The team's creation time.
	UpdatedAt   time.Time `json:"updated_at,omitempty"`                              // The team's last update time.
	Users       []*User   `gorm:"many2many:user_teams;" json:"users,omitempty"`      // The team's users.
}
