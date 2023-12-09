package service

import (
	model "github.com/elabosak233/pgshub/model/data"
	"github.com/elabosak233/pgshub/model/request"
	"github.com/elabosak233/pgshub/model/response"
)

type UserService interface {
	Create(req model.User) error
	Update(req request.UserUpdateRequest) error
	Delete(id string) error
	FindById(id string) (response.UserResponse, error)
	FindByUsername(username string) (response.UserResponse, error)
	VerifyPasswordById(id string, password string) bool
	VerifyPasswordByUsername(username string, password string) bool
	GetJwtTokenById(id string) string
	GetIdByJwtToken(token string) (string, error)
	FindAll() ([]response.UserResponse, error)
}
