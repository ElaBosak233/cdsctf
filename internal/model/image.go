package model

// Image is the image configuration for container platform.
// Because of the image is only a subsidiary table, it doesn't need the creation time or updated time.
type Image struct {
	ID          uint    `json:"id"`
	ChallengeID uint    `gorm:"not null;" json:"challenge_id"`
	Name        string  `gorm:"type:varchar(128);not null;" json:"name"` // A name of image is always repository + tag. (Such as "nginx:latest")
	CPULimit    float64 `gorm:"default:1;" json:"cpu_limit"`
	MemoryLimit int64   `gorm:"default:256;" json:"memory_limit"`
	Description string  `gorm:"type:text;" json:"description"`
	Ports       []*Port `json:"ports,omitempty"`
	Envs        []*Env  `json:"envs,omitempty"`
}
