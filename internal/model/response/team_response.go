package response

import (
	"github.com/elabosak233/cloudsdale/internal/model"
)

type TeamResponse struct {
	ID          uint          `json:"id"`
	Name        string        `json:"name"`
	Description string        `json:"description"`
	Email       string        `json:"email"`
	CaptainId   uint          `json:"captain_id"`
	IsLocked    bool          `json:"is_locked"`
	CreatedAt   int64         `json:"created_at"`
	UpdatedAt   int64         `json:"updated_at"`
	Users       []*model.User `json:"users,omitempty"`
	Captain     model.User    `json:"captain,omitempty"`
}

type TeamSimpleResponse struct {
	ID          uint   `json:"id"`
	Name        string `json:"name"`
	Description string `json:"description"`
	Email       string `json:"email"`
	CaptainId   uint   `json:"captain_id"`
	IsLocked    bool   `json:"is_locked"`
}
