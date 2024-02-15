package model

type Group struct {
	ID   uint   `json:"id"`
	Name string `gorm:"type:varchar(36);not null;unique;" json:"name"`
}
