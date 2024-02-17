package service

import (
	"errors"
	"github.com/elabosak233/cloudsdale/internal/config"
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/dto/request"
	"github.com/elabosak233/cloudsdale/internal/model/dto/response"
	"github.com/elabosak233/cloudsdale/internal/repository"
	"github.com/golang-jwt/jwt/v5"
	"github.com/mitchellh/mapstructure"
	"golang.org/x/crypto/bcrypt"
	"math"
	"time"
)

type IUserService interface {
	Create(req request.UserCreateRequest) (err error)
	Update(req request.UserUpdateRequest) (err error)
	Delete(id uint) error
	FindById(id uint) (response.UserResponse, error)
	FindByUsername(username string) (response.UserResponse, error)
	FindByEmail(email string) (user response.UserResponse, err error)
	VerifyPasswordById(id uint, password string) bool
	VerifyPasswordByUsername(username string, password string) bool
	GetJwtTokenById(user response.UserResponse) (tokenString string, err error)
	GetIdByJwtToken(token string) (id uint, err error)
	Find(req request.UserFindRequest) (users []response.UserResponse, pageCount int64, total int64, err error)
}

type UserService struct {
	UserRepository     repository.IUserRepository
	TeamRepository     repository.ITeamRepository
	UserTeamRepository repository.IUserTeamRepository
}

func NewUserService(appRepository *repository.Repository) IUserService {
	return &UserService{
		UserRepository:     appRepository.UserRepository,
		TeamRepository:     appRepository.TeamRepository,
		UserTeamRepository: appRepository.UserTeamRepository,
	}
}

func (t *UserService) GetJwtTokenById(user response.UserResponse) (tokenString string, err error) {
	jwtSecretKey := []byte(config.AppCfg().Jwt.SecretKey)
	pgsToken := jwt.NewWithClaims(jwt.SigningMethodHS256, jwt.MapClaims{
		"user_id": user.ID,
		"exp":     time.Now().Add(time.Duration(config.AppCfg().Jwt.Expiration) * time.Minute).Unix(),
	})
	return pgsToken.SignedString(jwtSecretKey)
}

func (t *UserService) GetIdByJwtToken(token string) (id uint, err error) {
	pgsToken, err := jwt.Parse(token, func(token *jwt.Token) (interface{}, error) {
		return []byte(config.AppCfg().Jwt.SecretKey), nil
	})
	if err != nil {
		return 0, err
	}
	if claims, ok := pgsToken.Claims.(jwt.MapClaims); ok && pgsToken.Valid {
		return uint(claims["user_id"].(float64)), nil
	} else {
		return 0, errors.New("无效 Token")
	}
}

func (t *UserService) Create(req request.UserCreateRequest) (err error) {
	hashedPassword, _ := bcrypt.GenerateFromPassword([]byte(req.Password), bcrypt.DefaultCost)
	userModel := model.User{
		Username: req.Username,
		Email:    req.Email,
		Nickname: req.Nickname,
		GroupID:  req.GroupID,
		Password: string(hashedPassword),
	}
	err = t.UserRepository.Insert(userModel)
	return err
}

func (t *UserService) Update(req request.UserUpdateRequest) (err error) {
	userModel := model.User{}
	_ = mapstructure.Decode(req, &userModel)
	if req.Password != "" {
		hashedPassword, _ := bcrypt.GenerateFromPassword([]byte(req.Password), bcrypt.DefaultCost)
		userModel.Password = string(hashedPassword)
	}
	err = t.UserRepository.Update(userModel)
	return err
}

func (t *UserService) Delete(id uint) error {
	err := t.UserRepository.Delete(id)
	err = t.UserTeamRepository.DeleteByUserId(id)
	return err
}

func (t *UserService) Find(req request.UserFindRequest) (users []response.UserResponse, pageCount int64, total int64, err error) {
	userResults, count, err := t.UserRepository.Find(req)
	for _, result := range userResults {
		var userResponse response.UserResponse
		_ = mapstructure.Decode(result, &userResponse)
		users = append(users, userResponse)
	}
	if req.Size >= 1 && req.Page >= 1 {
		pageCount = int64(math.Ceil(float64(count) / float64(req.Size)))
	} else {
		pageCount = 1
	}
	return users, pageCount, count, err
}

func (t *UserService) FindById(id uint) (response.UserResponse, error) {
	userData, err := t.UserRepository.FindById(id)
	userResp := response.UserResponse{}
	_ = mapstructure.Decode(userData, &userResp)
	return userResp, err
}

func (t *UserService) FindByUsername(username string) (response.UserResponse, error) {
	userData, err := t.UserRepository.FindByUsername(username)
	if err != nil {
		return response.UserResponse{}, errors.New("用户不存在")
	}
	userResp := response.UserResponse{}
	_ = mapstructure.Decode(userData, &userResp)
	return userResp, nil
}

func (t *UserService) FindByEmail(email string) (user response.UserResponse, err error) {
	userData, err := t.UserRepository.FindByEmail(email)
	if err != nil {
		return user, errors.New("用户不存在")
	}
	_ = mapstructure.Decode(userData, &user)
	return user, err
}

func (t *UserService) VerifyPasswordById(id uint, password string) bool {
	userData, err := t.UserRepository.FindById(id)
	err = bcrypt.CompareHashAndPassword([]byte(userData.Password), []byte(password))
	if err != nil {
		return false
	}
	return true
}

func (t *UserService) VerifyPasswordByUsername(username string, password string) bool {
	userData, err := t.UserRepository.FindByUsername(username)
	err = bcrypt.CompareHashAndPassword([]byte(userData.Password), []byte(password))
	if err != nil {
		return false
	}
	return true
}
