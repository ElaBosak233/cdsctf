package response

import (
	"github.com/elabosak233/cloudsdale/internal/model"
)

type UserResponse struct {
	ID        uint                  `json:"id"`
	Username  string                `json:"username"`
	Nickname  string                `json:"nickname"`
	Email     string                `json:"email"`
	GroupID   uint                  `json:"group_id"`
	Group     *model.Group          `json:"group"`
	CreatedAt int64                 `json:"created_at"`
	UpdatedAt int64                 `json:"updated_at"`
	Teams     []*TeamSimpleResponse `json:"teams,omitempty"`
}

type UserSimpleResponse struct {
	ID       uint   `xorm:"'id'" json:"id"`
	Username string `xorm:"'username'" json:"username"`
	Nickname string `xorm:"'nickname'" json:"nickname"`
}
