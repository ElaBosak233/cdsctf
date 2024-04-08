package model

// Nat is a model used to reveal the relationship between JeopardyImage ports and Container network port forwarding.
type Nat struct {
	ID          uint       `json:"id"`
	ContainerID uint       `gorm:"not null" json:"container_id"`
	Container   *Container `json:"container,omitempty"`
	SrcPort     int        `gorm:"not null" json:"src_port"` // Of Image
	DstPort     int        `gorm:"not null" json:"dst_port"` // Of Container
	Proxy       string     `json:"proxy"`                    // Of Platform
	Entry       string     `gorm:"type:varchar(128)" json:"entry"`
}
