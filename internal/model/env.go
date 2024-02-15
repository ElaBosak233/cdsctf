package model

type Env struct {
	ID      uint   `json:"id"`
	Key     string `gorm:"type:varchar(128);not null;" json:"key"`
	Value   string `gorm:"type:varchar(128);not null;" json:"value"`
	ImageID uint   `gorm:"not null;" json:"image_id"`
	Image   *Image `json:"image,omitempty"`
}
