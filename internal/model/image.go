package model

// Image is the image configuration for container platform.
// Because of the image is only a subsidiary table, it doesn't need the creation time or updated time.
type Image struct {
	ID          int64   `xorm:"'id' pk autoincr" json:"id"`
	ChallengeID int64   `xorm:"'challenge_id' notnull" json:"challenge_id"`
	Name        string  `xorm:"'name' varchar(128) notnull" json:"name"` // A name of image is always repository + tag. (Such as "nginx:latest")
	CPULimit    float64 `xorm:"'cpu_limit' default(1)" json:"cpu_limit"`
	MemoryLimit int64   `xorm:"'memory_limit' default(256)" json:"memory_limit"`
	Description string  `xorm:"'description' text" json:"description"`
	Ports       []Port  `xorm:"-" json:"ports"`
	Envs        []Env   `xorm:"-" json:"envs"`
}

func (i *Image) TableName() string {
	return "image"
}
