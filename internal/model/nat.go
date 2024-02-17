package model

// Nat is a model used to reveal the relationship between JeopardyImage ports and Instance network port forwarding.
type Nat struct {
	ID         uint      `json:"id"`
	InstanceID uint      `gorm:"not null" json:"instance_id"`
	Instance   *Instance `json:"instance,omitempty"`
	SrcPort    int       `gorm:"not null" json:"src_port"` // Of Image
	DstPort    int       `gorm:"not null" json:"dst_port"` // Of Instance
	Proxy      string    `json:"proxy"`                    // Of Platform
	Entry      string    `gorm:"type:varchar(128)" json:"entry"`
}
