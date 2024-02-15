package model

// FlagGen is the generated flag which is injected into the container.
// It will be generated when Flag's type is "dynamic".
type FlagGen struct {
	ID    uint   `json:"id"`
	Flag  string `gorm:"type:varchar(128);" json:"flag"`
	PodID uint   `gorm:"not null;" json:"pod_id"`
	Pod   Pod    `json:"pod"`
}
