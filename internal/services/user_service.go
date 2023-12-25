package services

import (
	model "github.com/elabosak233/pgshub/internal/models/data"
	"github.com/elabosak233/pgshub/internal/models/request"
	"github.com/elabosak233/pgshub/internal/models/response"
)

type UserService interface {
	Create(req model.User) error
	Update(req request.UserUpdateRequest) error
	Delete(id string) error
	FindById(id string) (response.UserResponse, error)
	FindByUsername(username string) (response.UserResponse, error)
	VerifyPasswordById(id string, password string) bool
	VerifyPasswordByUsername(username string, password string) bool
	GetJwtTokenById(user response.UserResponse) (tokenString string, err error)
	GetIdByJwtToken(token string) (id string, err error)
	FindAll() ([]response.UserResponse, error)
}
