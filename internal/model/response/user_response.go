package response

import (
	"github.com/elabosak233/cloudsdale/internal/model"
)

type UserResponse struct {
	ID        uint          `json:"id"`
	Username  string        `json:"username"`
	Nickname  string        `json:"nickname"`
	Email     string        `json:"email"`
	GroupID   uint          `json:"group_id"`
	Group     *model.Group  `json:"group"`
	CreatedAt int64         `json:"created_at"`
	UpdatedAt int64         `json:"updated_at"`
	Teams     []*model.Team `json:"teams,omitempty"`
}
