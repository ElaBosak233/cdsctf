package entity

// Nat is an entity used to reveal the relationship between Image ports and Container network port forwarding.
type Nat struct {
	NatID       int64  `xorm:"'id' pk autoincr" json:"id"`
	ContainerID int64  `xorm:"'container_id' notnull" json:"container_id"`
	SrcPort     int    `xorm:"'src_port' notnull" json:"src_port"`
	DstPort     int    `xorm:"'dst_port' notnull" json:"dst_port"`
	Entry       string `xorm:"'entry' varchar(128)" json:"entry"`
}

func (n *Nat) TableName() string {
	return "nat"
}
