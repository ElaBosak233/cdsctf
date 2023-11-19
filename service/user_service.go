package service

import (
	req "github.com/elabosak233/pgshub/model/request/account"
	"github.com/elabosak233/pgshub/model/response"
)

type UserService interface {
	Create(req req.CreateUserRequest) error
	Update(req req.UpdateUserRequest) error
	Delete(id string) error
	FindById(id string) response.UserResponse
	FindByUsername(username string) response.UserResponse
	VerifyPasswordById(id string, password string) bool
	GetJwtTokenById(id string) string
	FindAll() []response.UserResponse
}
