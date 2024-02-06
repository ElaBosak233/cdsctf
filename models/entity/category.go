package entity

import "time"

// Category is the category of the challenge.
type Category struct {
	CategoryID  int64     `xorm:"'id' pk autoincr" json:"id"`                          // The category's id. As primary key.
	Name        string    `xorm:"'name' varchar(32) notnull unique" json:"name"`       // The category's name.
	Description string    `xorm:"'description' text" json:"description"`               // The category's description.
	ColorHex    string    `xorm:"'color_hex' varchar(7)" json:"color_hex"`             // The category's theme color. (Such as Rainbow Dash's color is "#60AEE4")
	Mdi         string    `xorm:"'mdi' varchar(32) default('fingerprint')" json:"mdi"` // The category's icon. (Based on Material Design Icons, Reference site: https://pictogrammers.com/library/mdi/) (Such as "fingerprint": https://pictogrammers.com/library/mdi/icon/fingerprint/)
	CreatedAt   time.Time `xorm:"'created_at' created" json:"created_at"`              // The category's creation time.
	UpdatedAt   time.Time `xorm:"'updated_at' updated" json:"updated_at"`              // The category's last update time.
}

func (c *Category) TableName() string {
	return "category"
}
