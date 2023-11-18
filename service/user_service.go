package service

import (
	request2 "github.com/elabosak233/pgshub/model/request"
	"github.com/elabosak233/pgshub/model/response"
)

type UserService interface {
	Create(req request2.CreateUserRequest)
	Update(req request2.UpdateUserRequest)
	Delete(id string)
	FindById(id string) response.UserResponse
	FindByUsername(username string) response.UserResponse
	FindAll() []response.UserResponse
}
