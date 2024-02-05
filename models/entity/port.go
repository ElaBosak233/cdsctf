package entity

import "time"

// Port is the mapping between the Image and the exposed port of the Image.
type Port struct {
	PortID      int64     `xorm:"'id' pk autoincr" json:"id"`                   // The port's id. As primary key.
	ImageID     int64     `xorm:"'image_id' notnull" json:"image_id"`           // The Image which the port belongs to.
	Value       int       `xorm:"'value' notnull" json:"value"`                 // The port number.
	Description string    `xorm:"'description' varchar(32)" json:"description"` // The port's description.
	CreatedAt   time.Time `xorm:"'created_at' created" json:"created_at"`       // The port's creation time.
	UpdatedAt   time.Time `xorm:"'updated_at' updated" json:"updated_at"`       // The port's last update time.
}
