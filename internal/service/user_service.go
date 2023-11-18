package service

import (
	"github.com/elabosak233/pgshub/internal/model/request"
	"github.com/elabosak233/pgshub/internal/model/response"
)

type UserService interface {
	Create(req request.CreateUserRequest)
	Update(req request.UpdateUserRequest)
	Delete(id string)
	FindById(id string) response.UserResponse
	FindByUsername(username string) response.UserResponse
	FindAll() []response.UserResponse
}
