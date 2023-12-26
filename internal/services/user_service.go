package services

import (
	"github.com/elabosak233/pgshub/internal/models/request"
	"github.com/elabosak233/pgshub/internal/models/response"
)

type UserService interface {
	Create(req request.UserCreateRequest) (err error)
	Update(req request.UserUpdateRequest) error
	Delete(id string) error
	FindById(id string) (response.UserResponse, error)
	FindByUsername(username string) (response.UserResponse, error)
	FindByEmail(email string) (user response.UserResponse, err error)
	VerifyPasswordById(id string, password string) bool
	VerifyPasswordByUsername(username string, password string) bool
	GetJwtTokenById(user response.UserResponse) (tokenString string, err error)
	GetIdByJwtToken(token string) (id string, err error)
	Find(req request.UserFindRequest) (users []response.UserResponse, pageCount int64, err error)
}
