package model

type Instance struct {
	ID      uint   `json:"id,omitempty"`
	ImageID uint   `gorm:"not null" json:"image_id,omitempty"`
	Image   *Image `json:"image,omitempty"`
	PodID   uint   `gorm:"not null" json:"pod_id,omitempty"`
	Pod     *Pod   `json:"pod,omitempty"`
	Nats    []*Nat `json:"nats,omitempty"`
}
