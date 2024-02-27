package model

type Group struct {
	ID          uint    `json:"id"`
	Name        string  `gorm:"type:varchar(36);not null;unique;" json:"name"`
	DisplayName string  `gorm:"type:varchar(36);not null;" json:"display_name"`
	Description string  `gorm:"type:text" json:"description"`
	Users       []*User `json:"users,omitempty"`
}
