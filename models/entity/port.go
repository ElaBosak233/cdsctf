package entity

// Port is the mapping between the JeopardyImage and the exposed port of the JeopardyImage.
// Because of the flag is only a subsidiary table, it doesn't need the creation time or updated time.
type Port struct {
	ID          int64  `xorm:"'id' pk autoincr" json:"id"`                          // The port's id. As primary key.
	ImageID     int64  `xorm:"'image_id' notnull unique(port_idx)" json:"image_id"` // The JeopardyImage which the port belongs to.
	Value       int    `xorm:"'value' notnull unique(port_idx)" json:"value"`       // The port number.
	Description string `xorm:"'description' varchar(32)" json:"description"`        // The port's description.
}

func (p *Port) TableName() string {
	return "port"
}
