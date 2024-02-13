package model

type Instance struct {
	ID      int64 `xorm:"'id' pk autoincr" json:"id,omitempty"`
	ImageID int64 `xorm:"'image_id' notnull" json:"image_id,omitempty"`
	PodID   int64 `xorm:"'pod_id' notnull" json:"pod_id,omitempty"`

	Nats  []Nat  `xorm:"-" json:"nats,omitempty"`
	Image *Image `xorm:"-" json:"image,omitempty"`
}

func (c *Instance) TableName() string {
	return "instance"
}
