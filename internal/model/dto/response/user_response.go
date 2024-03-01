package response

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"time"
)

type UserResponse struct {
	ID        uint                  `json:"id"`
	Username  string                `json:"username"`
	Nickname  string                `json:"nickname"`
	Email     string                `json:"email"`
	GroupID   uint                  `json:"group_id"`
	Group     *model.Group          `json:"group"`
	CreatedAt *time.Time            `json:"created_at"`
	UpdatedAt *time.Time            `json:"updated_at"`
	Teams     []*TeamSimpleResponse `json:"teams,omitempty"`
}

type UserSimpleResponse struct {
	ID       uint   `xorm:"'id'" json:"id"`
	Username string `xorm:"'username'" json:"username"`
	Nickname string `xorm:"'nickname'" json:"nickname"`
}
