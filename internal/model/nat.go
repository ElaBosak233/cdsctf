package model

type Nat struct {
	ID      uint   `json:"id"`
	PodID   uint   `gorm:"not null" json:"pod_id"`
	Pod     *Pod   `json:"pod,omitempty"`
	SrcPort int    `gorm:"not null" json:"src_port"` // Of image
	DstPort int    `gorm:"not null" json:"dst_port"` // Of pod
	Proxy   string `json:"proxy"`                    // Of platform
	Entry   string `gorm:"type:varchar(128)" json:"entry"`
}
