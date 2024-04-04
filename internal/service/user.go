package service

import (
	"errors"
	"github.com/elabosak233/cloudsdale/internal/captcha"
	"github.com/elabosak233/cloudsdale/internal/config"
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"github.com/elabosak233/cloudsdale/internal/repository"
	"github.com/golang-jwt/jwt/v5"
	"github.com/mitchellh/mapstructure"
	"golang.org/x/crypto/bcrypt"
	"math"
	"strings"
	"time"
)

type IUserService interface {
	Create(req request.UserCreateRequest) (err error)
	Register(req request.UserRegisterRequest) (err error)
	Update(req request.UserUpdateRequest) (err error)
	Delete(id uint) error
	VerifyPasswordByUsername(username string, password string) bool
	GetJwtTokenByID(user model.User) (tokenString string, err error)
	GetIDByJwtToken(token string) (id uint, err error)
	Find(req request.UserFindRequest) (users []model.User, pages int64, total int64, err error)
}

type UserService struct {
	userRepository     repository.IUserRepository
	teamRepository     repository.ITeamRepository
	userTeamRepository repository.IUserTeamRepository
}

func NewUserService(appRepository *repository.Repository) IUserService {
	return &UserService{
		userRepository:     appRepository.UserRepository,
		teamRepository:     appRepository.TeamRepository,
		userTeamRepository: appRepository.UserTeamRepository,
	}
}

func (t *UserService) GetJwtTokenByID(user model.User) (tokenString string, err error) {
	jwtSecretKey := []byte(config.AppCfg().Gin.Jwt.SecretKey)
	pgsToken := jwt.NewWithClaims(jwt.SigningMethodHS256, jwt.MapClaims{
		"user_id": user.ID,
		"exp":     time.Now().Add(time.Duration(config.AppCfg().Gin.Jwt.Expiration) * time.Minute).Unix(),
	})
	return pgsToken.SignedString(jwtSecretKey)
}

func (t *UserService) GetIDByJwtToken(token string) (id uint, err error) {
	pgsToken, err := jwt.Parse(token, func(token *jwt.Token) (interface{}, error) {
		return []byte(config.AppCfg().Gin.Jwt.SecretKey), nil
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
		Username: strings.ToLower(req.Username),
		Email:    strings.ToLower(req.Email),
		Nickname: req.Nickname,
		GroupID:  req.GroupID,
		Password: string(hashedPassword),
	}
	err = t.userRepository.Create(userModel)
	return err
}

func (t *UserService) Register(req request.UserRegisterRequest) (err error) {
	hashedPassword, _ := bcrypt.GenerateFromPassword([]byte(req.Password), bcrypt.DefaultCost)
	capt := captcha.NewCaptcha()
	success, err := capt.Verify(req.CaptchaToken, req.RemoteIP)
	if success {
		userModel := model.User{
			Username: strings.ToLower(req.Username),
			Email:    strings.ToLower(req.Email),
			Nickname: req.Nickname,
			GroupID:  3,
			Password: string(hashedPassword),
		}
		err = t.userRepository.Create(userModel)
	}
	return err
}

func (t *UserService) Update(req request.UserUpdateRequest) (err error) {
	userModel := model.User{}
	_ = mapstructure.Decode(req, &userModel)
	if req.Password != "" {
		hashedPassword, _ := bcrypt.GenerateFromPassword([]byte(req.Password), bcrypt.DefaultCost)
		userModel.Password = string(hashedPassword)
	}
	if req.Username != "" {
		userModel.Username = strings.ToLower(req.Username)
	}
	if req.Email != "" {
		userModel.Email = strings.ToLower(req.Email)
	}
	err = t.userRepository.Update(userModel)
	return err
}

func (t *UserService) Delete(id uint) error {
	err := t.userRepository.Delete(id)
	err = t.userTeamRepository.DeleteByUserId(id)
	return err
}

func (t *UserService) Find(req request.UserFindRequest) (users []model.User, pages int64, total int64, err error) {
	users, count, err := t.userRepository.Find(req)
	for index := range users {
		users[index].Simplify()
	}
	if req.Size >= 1 && req.Page >= 1 {
		pages = int64(math.Ceil(float64(count) / float64(req.Size)))
	} else {
		pages = 1
	}
	return users, pages, count, err
}

func (t *UserService) VerifyPasswordById(id uint, password string) bool {
	userData, err := t.userRepository.FindById(id)
	err = bcrypt.CompareHashAndPassword([]byte(userData.Password), []byte(password))
	if err != nil {
		return false
	}
	return true
}

func (t *UserService) VerifyPasswordByUsername(username string, password string) bool {
	userData, err := t.userRepository.FindByUsername(strings.ToLower(username))
	err = bcrypt.CompareHashAndPassword([]byte(userData.Password), []byte(password))
	if err != nil {
		return false
	}
	return true
}
