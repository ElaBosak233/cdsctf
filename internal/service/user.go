package service

import (
	"errors"
	"github.com/elabosak233/cloudsdale/internal/captcha"
	"github.com/elabosak233/cloudsdale/internal/config"
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"github.com/elabosak233/cloudsdale/internal/model/response"
	"github.com/elabosak233/cloudsdale/internal/repository"
	"github.com/golang-jwt/jwt/v5"
	"github.com/mitchellh/mapstructure"
	"golang.org/x/crypto/bcrypt"
	"math"
	"time"
)

type IUserService interface {
	Create(req request.UserCreateRequest) (err error)
	Register(req request.UserRegisterRequest) (err error)
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

func (t *UserService) GetJwtTokenById(user response.UserResponse) (tokenString string, err error) {
	jwtSecretKey := []byte(config.AppCfg().Gin.Jwt.SecretKey)
	pgsToken := jwt.NewWithClaims(jwt.SigningMethodHS256, jwt.MapClaims{
		"user_id": user.ID,
		"exp":     time.Now().Add(time.Duration(config.AppCfg().Gin.Jwt.Expiration) * time.Minute).Unix(),
	})
	return pgsToken.SignedString(jwtSecretKey)
}

func (t *UserService) GetIdByJwtToken(token string) (id uint, err error) {
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
		Username: req.Username,
		Email:    req.Email,
		Nickname: req.Nickname,
		GroupID:  req.GroupID,
		Password: string(hashedPassword),
	}
	err = t.userRepository.Insert(userModel)
	return err
}

func (t *UserService) Register(req request.UserRegisterRequest) (err error) {
	hashedPassword, _ := bcrypt.GenerateFromPassword([]byte(req.Password), bcrypt.DefaultCost)
	capt := captcha.NewCaptcha()
	success, err := capt.Verify(req.CaptchaToken, req.RemoteIP)
	if success {
		userModel := model.User{
			Username: req.Username,
			Email:    req.Email,
			Nickname: req.Nickname,
			GroupID:  3,
			Password: string(hashedPassword),
		}
		err = t.userRepository.Insert(userModel)
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
	err = t.userRepository.Update(userModel)
	return err
}

func (t *UserService) Delete(id uint) error {
	err := t.userRepository.Delete(id)
	err = t.userTeamRepository.DeleteByUserId(id)
	return err
}

func (t *UserService) Find(req request.UserFindRequest) (users []response.UserResponse, pageCount int64, total int64, err error) {
	userResults, count, err := t.userRepository.Find(req)
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
	userData, err := t.userRepository.FindById(id)
	userResp := response.UserResponse{}
	_ = mapstructure.Decode(userData, &userResp)
	return userResp, err
}

func (t *UserService) FindByUsername(username string) (response.UserResponse, error) {
	userData, err := t.userRepository.FindByUsername(username)
	if err != nil {
		return response.UserResponse{}, errors.New("用户不存在")
	}
	userResp := response.UserResponse{}
	_ = mapstructure.Decode(userData, &userResp)
	return userResp, nil
}

func (t *UserService) FindByEmail(email string) (user response.UserResponse, err error) {
	userData, err := t.userRepository.FindByEmail(email)
	if err != nil {
		return user, errors.New("用户不存在")
	}
	_ = mapstructure.Decode(userData, &user)
	return user, err
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
	userData, err := t.userRepository.FindByUsername(username)
	err = bcrypt.CompareHashAndPassword([]byte(userData.Password), []byte(password))
	if err != nil {
		return false
	}
	return true
}
