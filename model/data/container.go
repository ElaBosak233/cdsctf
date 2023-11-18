package data

import "time"

type Container struct {
	Id          string    `xorm:"pk unique 'id' notnull" json:"id"`
	Image       string    `xorm:"text 'image' notnull" json:"image"`
	ContainerId string    `xorm:"text 'container_id'" json:"container_id"`
	Status      string    `xorm:"text 'status'" json:"status"`
	CreatedAt   time.Time `xorm:"created 'created_at'" json:"created_at"`
}
