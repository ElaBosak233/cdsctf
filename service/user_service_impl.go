package service

import (
	"github.com/dgrijalva/jwt-go"
	model "github.com/elabosak233/pgshub/model/data"
	"github.com/elabosak233/pgshub/model/misc"
	req "github.com/elabosak233/pgshub/model/request/account"
	"github.com/elabosak233/pgshub/model/response"
	"github.com/elabosak233/pgshub/repository"
	"github.com/elabosak233/pgshub/utils"
	"github.com/go-playground/validator/v10"
	"github.com/google/uuid"
	"golang.org/x/crypto/bcrypt"
	"time"
)

type UserServiceImpl struct {
	UserRepository repository.UserRepository
	Validate       *validator.Validate
}

func NewUserServiceImpl(appRepository repository.AppRepository) UserService {
	return &UserServiceImpl{
		UserRepository: appRepository.UserRepository,
		Validate:       validator.New(),
	}
}

func (t *UserServiceImpl) GetJwtTokenById(id string) string {
	expirationTime := time.Now().Add(time.Duration(utils.Cfg.Jwt.ExpirationTime) * time.Minute)
	jwtSecretKey := []byte(utils.Cfg.Jwt.SecretKey)
	claims := &misc.Claims{
		Id: id,
		StandardClaims: jwt.StandardClaims{
			ExpiresAt: expirationTime.Unix(),
		},
	}
	token := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)
	tokenString, err := token.SignedString(jwtSecretKey)
	utils.ErrorPanic(err)
	return tokenString
}

// Create implements UserService
func (t *UserServiceImpl) Create(req req.CreateUserRequest) error {
	err := t.Validate.Struct(req)
	utils.ErrorPanic(err)
	hashedPassword, _ := bcrypt.GenerateFromPassword([]byte(req.Password), bcrypt.DefaultCost)
	userModel := model.User{
		Id:       uuid.NewString(),
		Username: req.Username,
		Email:    req.Email,
		Password: string(hashedPassword),
	}
	err = t.UserRepository.Insert(userModel)
	return err
}

// Update implements UserService
func (t *UserServiceImpl) Update(req req.UpdateUserRequest) error {
	userData, err := t.UserRepository.FindById(req.Id)
	if err != nil {
		return err
	}
	userData.Username = req.Username
	err = bcrypt.CompareHashAndPassword([]byte(userData.Password), []byte(req.Password))
	if err != nil {
		return err
	}
	err = t.UserRepository.Update(userData)
	return err
}

// Delete implements UserService
func (t *UserServiceImpl) Delete(id string) error {
	err := t.UserRepository.Delete(id)
	return err
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

func (t *UserServiceImpl) VerifyPasswordById(id string, password string) bool {
	userData, err := t.UserRepository.FindById(id)
	err = bcrypt.CompareHashAndPassword([]byte(userData.Password), []byte(password))
	if err != nil {
		return false
	} else {
		return true
	}
}
