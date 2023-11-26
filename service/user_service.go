package service

import (
	model "github.com/elabosak233/pgshub/model/data"
	"time"
)

type UserResponse struct {
	Id        string    `json:"id"`
	Username  string    `json:"username"`
	Email     string    `json:"email"`
	CreatedAt time.Time `json:"created_at"`
	UpdatedAt time.Time `json:"updated_at"`
}

type UserUpdateRequest struct {
	Id       string `binding:"required" json:"id"`
	Username string `binding:"required,max=20,min=3" json:"username"`
	Password string `binding:"required,min=6" json:"password"`
	Email    string `binding:"email" json:"email"`
}

type UserService interface {
	Create(req model.User) error
	Update(req UserUpdateRequest) error
	Delete(id string) error
	FindById(id string) (UserResponse, error)
	FindByUsername(username string) (UserResponse, error)
	VerifyPasswordById(id string, password string) bool
	VerifyPasswordByUsername(username string, password string) bool
	GetJwtTokenById(id string) string
	FindAll() ([]UserResponse, error)
}
