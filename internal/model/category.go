package model

import (
	"gorm.io/gorm"
)

// Category is the category of the challenge.
type Category struct {
	ID          uint   `json:"id"`                                                  // The category's id. As primary key.
	Name        string `gorm:"type:varchar(32);not null;unique" json:"name"`        // The category's name.
	Description string `gorm:"type:text" json:"description"`                        // The category's description.
	Color       string `gorm:"type:varchar(7)" json:"color"`                        // The category's theme color. (Such as Rainbow Dash's color is "#60AEE4")
	Icon        string `gorm:"type:varchar(32);default:'fingerprint';" json:"icon"` // The category's icon. (Based on Material Design Icons, Reference site: https://pictogrammers.com/library/mdi/) (Such as "fingerprint": https://pictogrammers.com/library/mdi/icon/fingerprint/)
	CreatedAt   int64  `gorm:"autoUpdateTime:milli" json:"created_at,omitempty"`    // The category's creation time.
	UpdatedAt   int64  `gorm:"autoUpdateTime:milli" json:"updated_at,omitempty"`    // The category's last update time.
}

func (c *Category) BeforeDelete(db *gorm.DB) (err error) {
	var challenges []Challenge
	db.Table("challenges").Where("category_id = ?", c.ID).Find(&challenges)
	for _, challenge := range challenges {
		db.Table("challenges").Delete(&challenge)
	}
	return nil
}
