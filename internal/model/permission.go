package model

type Permission struct {
	ID   uint   `json:"id"`
	Name string `gorm:"type:varchar(128);not null;unique;" json:"name"`
}
