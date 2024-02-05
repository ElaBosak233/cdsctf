package entity

import "time"

type Image struct {
	ImageID     int64     `xorm:"'id' pk autoincr" json:"id"`
	ChallengeID int64     `xorm:"'challenge_id' notnull" json:"challenge_id"`
	Name        string    `xorm:"'name' varchar(128) notnull" json:"name"` // A name of image is always repository + tag. (Such as "nginx:latest")
	CPULimit    float64   `xorm:"'cpu_limit' default(1)" json:"cpu_limit"`
	MemoryLimit int64     `xorm:"'memory_limit' default(256)" json:"memory_limit"`
	Description string    `xorm:"'description' text" json:"description"`
	CreatedAt   time.Time `xorm:"'created_at' created" json:"created_at"`
	UpdatedAt   time.Time `xorm:"'updated_at' updated" json:"updated_at"`
}
