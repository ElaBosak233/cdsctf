package model

type Group struct {
	ID          uint    `json:"id"`
	Name        string  `gorm:"type:varchar(36);not null;unique;" json:"name"`
	Description string  `gorm:"type:text" json:"description"`
	Level       int64   `json:"level"`
	Users       []*User `json:"users,omitempty"`
}
