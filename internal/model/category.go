package model

import "time"

// Category is the category of the challenge.
type Category struct {
	ID          uint      `json:"id"`                                                  // The category's id. As primary key.
	Name        string    `gorm:"type:varchar(32);not null;unique" json:"name"`        // The category's name.
	Description string    `gorm:"type:text" json:"description"`                        // The category's description.
	Color       string    `gorm:"type:varchar(7)" json:"color"`                        // The category's theme color. (Such as Rainbow Dash's color is "#60AEE4")
	Icon        string    `gorm:"type:varchar(32);default:'fingerprint';" json:"icon"` // The category's icon. (Based on Material Design Icons, Reference site: https://pictogrammers.com/library/mdi/) (Such as "fingerprint": https://pictogrammers.com/library/mdi/icon/fingerprint/)
	CreatedAt   time.Time `json:"created_at"`                                          // The category's creation time.
	UpdatedAt   time.Time `json:"updated_at"`                                          // The category's last update time.
}
