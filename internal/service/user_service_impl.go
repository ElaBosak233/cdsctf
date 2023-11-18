package service

import (
	"github.com/elabosak233/pgshub/internal/model/data"
	"github.com/elabosak233/pgshub/internal/model/request"
	"github.com/elabosak233/pgshub/internal/model/response"
	"github.com/elabosak233/pgshub/internal/repository"
	"github.com/elabosak233/pgshub/internal/utils"
	"github.com/go-playground/validator/v10"
	"github.com/satori/go.uuid"
	"golang.org/x/crypto/bcrypt"
)

type UserServiceImpl struct {
	UserRepository repository.UserRepository
	Validate       *validator.Validate
}

func NewUserServiceImpl(appRepository repository.AppRepository, validate *validator.Validate) UserService {
	return &UserServiceImpl{
		UserRepository: appRepository.UserRepository,
		Validate:       validate,
	}
}

// Create implements UserService
func (t *UserServiceImpl) Create(req request.CreateUserRequest) {
	err := t.Validate.Struct(req)
	utils.ErrorPanic(err)
	hashedPassword, _ := bcrypt.GenerateFromPassword([]byte(req.Password), bcrypt.DefaultCost)
	userModel := data.User{
		Id:       uuid.NewV4().String(),
		Username: req.Username,
		Password: string(hashedPassword),
	}
	t.UserRepository.Save(userModel)
}

// Delete implements UserService
func (t *UserServiceImpl) Delete(id string) {
	t.UserRepository.Delete(id)
}

// FindAll implements UserService
func (t *UserServiceImpl) FindAll() []response.UserResponse {
	result := t.UserRepository.FindAll()

	var users []response.UserResponse
	for _, value := range result {
		user := response.UserResponse{
			Id:       value.Id,
			Username: value.Username,
			GroupIds: value.GroupIds,
		}
		users = append(users, user)
	}

	return users
}

// FindById implements UserService
func (t *UserServiceImpl) FindById(id string) response.UserResponse {
	userData, err := t.UserRepository.FindById(id)
	utils.ErrorPanic(err)
	user := response.UserResponse{
		Id:       userData.Id,
		Username: userData.Username,
		GroupIds: userData.GroupIds,
	}
	return user
}

func (t *UserServiceImpl) FindByUsername(username string) response.UserResponse {
	userData, err := t.UserRepository.FindByUsername(username)
	utils.ErrorPanic(err)
	user := response.UserResponse{
		Id:       userData.Id,
		Username: userData.Username,
		GroupIds: userData.GroupIds,
	}
	return user
}

// Update implements UserService
func (t *UserServiceImpl) Update(req request.UpdateUserRequest) {
	userData, err := t.UserRepository.FindById(req.Id)
	utils.ErrorPanic(err)
	userData.Username = req.Username
	err = bcrypt.CompareHashAndPassword([]byte(userData.Password), []byte(req.Password))
	utils.ErrorPanic(err)
	t.UserRepository.Update(userData)
}
