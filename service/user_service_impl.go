package service

import (
	"errors"
	"github.com/dgrijalva/jwt-go"
	model "github.com/elabosak233/pgshub/model/data"
	"github.com/elabosak233/pgshub/model/misc"
	"github.com/elabosak233/pgshub/model/request"
	"github.com/elabosak233/pgshub/model/response"
	"github.com/elabosak233/pgshub/repository"
	"github.com/elabosak233/pgshub/utils"
	"github.com/google/uuid"
	"github.com/mitchellh/mapstructure"
	"github.com/spf13/viper"
	"golang.org/x/crypto/bcrypt"
	"time"
)

type UserServiceImpl struct {
	UserRepository repository.UserRepository
}

func NewUserServiceImpl(appRepository *repository.AppRepository) UserService {
	return &UserServiceImpl{
		UserRepository: appRepository.UserRepository,
	}
}

func (t *UserServiceImpl) GetJwtTokenById(id string) string {
	expirationTime := time.Now().Add(time.Duration(viper.GetInt("Jwt.ExpirationTime")) * time.Minute)
	jwtSecretKey := []byte(viper.GetString("Jwt.SecretKey"))
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
func (t *UserServiceImpl) Create(req model.User) error {
	hashedPassword, _ := bcrypt.GenerateFromPassword([]byte(req.Password), bcrypt.DefaultCost)
	userModel := model.User{
		UserId:   uuid.NewString(),
		Username: req.Username,
		Email:    req.Email,
		Password: string(hashedPassword),
	}
	err := t.UserRepository.Insert(userModel)
	return err
}

// Update implements UserService
func (t *UserServiceImpl) Update(req request.UserUpdateRequest) error {
	userData, err := t.UserRepository.FindById(req.Id)
	if err != nil || userData.UserId == "" {
		return errors.New("用户不存在")
	}
	userModel := model.User{}
	_ = mapstructure.Decode(req, &userModel)
	err = t.UserRepository.Update(userModel)
	return err
}

// Delete implements UserService
func (t *UserServiceImpl) Delete(id string) error {
	err := t.UserRepository.Delete(id)
	return err
}

// FindAll implements UserService
func (t *UserServiceImpl) FindAll() ([]response.UserResponse, error) {
	result := t.UserRepository.FindAll()
	var users []response.UserResponse
	for _, value := range result {
		userResp := response.UserResponse{}
		_ = mapstructure.Decode(value, &userResp)
		userResp.CreatedAt = value.CreatedAt
		userResp.UpdatedAt = value.UpdatedAt
		users = append(users, userResp)
	}
	return users, nil
}

// FindById implements UserService
func (t *UserServiceImpl) FindById(id string) (response.UserResponse, error) {
	userData, err := t.UserRepository.FindById(id)
	if err != nil {
		return response.UserResponse{}, errors.New("用户不存在")
	}
	userResp := response.UserResponse{}
	_ = mapstructure.Decode(userData, &userResp)
	userResp.CreatedAt = userData.CreatedAt
	userResp.UpdatedAt = userData.UpdatedAt
	return userResp, nil
}

func (t *UserServiceImpl) FindByUsername(username string) (response.UserResponse, error) {
	userData, err := t.UserRepository.FindByUsername(username)
	if err != nil {
		return response.UserResponse{}, errors.New("用户不存在")
	}
	userResp := response.UserResponse{}
	_ = mapstructure.Decode(userData, &userResp)
	userResp.CreatedAt = userData.CreatedAt
	userResp.UpdatedAt = userData.UpdatedAt
	return userResp, nil
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

func (t *UserServiceImpl) VerifyPasswordByUsername(username string, password string) bool {
	userData, err := t.UserRepository.FindByUsername(username)
	err = bcrypt.CompareHashAndPassword([]byte(userData.Password), []byte(password))
	if err != nil {
		return false
	} else {
		return true
	}
}
