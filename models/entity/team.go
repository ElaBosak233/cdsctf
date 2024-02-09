package entity

import "time"

type Team struct {
	ID          int64      `xorm:"'id' pk autoincr" json:"id"`                                    // The team's id. As primary key.
	Name        string     `xorm:"'name' nvarchar(36) notnull" json:"name"`                       // The team's name.
	Description string     `xorm:"'description' text" json:"description"`                         // The team's description.
	CaptainID   int64      `xorm:"'captain_id' notnull" json:"captain_id,omitempty"`              // The captain's id.
	IsLocked    *bool      `xorm:"'is_locked' notnull default(false)" json:"is_locked,omitempty"` // Whether the team is locked. (true/false)
	CreatedAt   *time.Time `xorm:"'created_at' created" json:"created_at,omitempty"`              // The team's creation time.
	UpdatedAt   *time.Time `xorm:"'updated_at' updated" json:"updated_at,omitempty"`              // The team's last update time.
}

func (t *Team) TableName() string {
	return "team"
}
