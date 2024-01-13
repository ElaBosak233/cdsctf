package services

import (
	"errors"
	model "github.com/elabosak233/pgshub/internal/models/data"
	"github.com/elabosak233/pgshub/internal/models/request"
	"github.com/elabosak233/pgshub/internal/models/response"
	"github.com/elabosak233/pgshub/internal/repositories"
	"github.com/elabosak233/pgshub/internal/repositories/relations"
	jwt "github.com/golang-jwt/jwt/v5"
	"github.com/mitchellh/mapstructure"
	"github.com/spf13/viper"
	"golang.org/x/crypto/bcrypt"
	"math"
	"time"
)

type UserService interface {
	Create(req request.UserCreateRequest) (err error)
	Update(req request.UserUpdateRequest) error
	Delete(id int64) error
	FindById(id int64) (response.UserResponse, error)
	FindByUsername(username string) (response.UserResponse, error)
	FindByEmail(email string) (user response.UserResponse, err error)
	VerifyPasswordById(id int64, password string) bool
	VerifyPasswordByUsername(username string, password string) bool
	GetJwtTokenById(user response.UserResponse) (tokenString string, err error)
	GetIdByJwtToken(token string) (id int64, err error)
	Find(req request.UserFindRequest) (users []response.UserResponse, pageCount int64, err error)
}

type UserServiceImpl struct {
	UserRepository     repositories.UserRepository
	UserTeamRepository relations.UserTeamRepository
}

func NewUserServiceImpl(appRepository *repositories.AppRepository) UserService {
	return &UserServiceImpl{
		UserRepository:     appRepository.UserRepository,
		UserTeamRepository: appRepository.UserTeamRepository,
	}
}

func (t *UserServiceImpl) GetJwtTokenById(user response.UserResponse) (tokenString string, err error) {
	jwtSecretKey := []byte(viper.GetString("jwt.secret_key"))
	pgsToken := jwt.NewWithClaims(jwt.SigningMethodHS256, jwt.MapClaims{
		"user_id": user.UserId,
		"exp":     time.Now().Add(time.Duration(viper.GetInt("jwt.expiration")) * time.Minute).Unix(),
	})
	return pgsToken.SignedString(jwtSecretKey)
}

func (t *UserServiceImpl) GetIdByJwtToken(token string) (id int64, err error) {
	pgsToken, err := jwt.Parse(token, func(token *jwt.Token) (interface{}, error) {
		return []byte(viper.GetString("jwt.secret_key")), nil
	})
	if err != nil {
		return 0, err
	}
	if claims, ok := pgsToken.Claims.(jwt.MapClaims); ok && pgsToken.Valid {
		return int64(claims["user_id"].(float64)), nil
	} else {
		return 0, errors.New("无效 Token")
	}
}

func (t *UserServiceImpl) Create(req request.UserCreateRequest) (err error) {
	hashedPassword, _ := bcrypt.GenerateFromPassword([]byte(req.Password), bcrypt.DefaultCost)
	userModel := model.User{
		Username: req.Username,
		Email:    req.Email,
		Name:     req.Name,
		Role:     req.Role,
		Password: string(hashedPassword),
	}
	err = t.UserRepository.Insert(userModel)
	return err
}

func (t *UserServiceImpl) Update(req request.UserUpdateRequest) error {
	hashedPassword, _ := bcrypt.GenerateFromPassword([]byte(req.Password), bcrypt.DefaultCost)
	userData, err := t.UserRepository.FindById(req.UserId)
	if err != nil || userData.UserId == 0 {
		return errors.New("用户不存在")
	}
	userModel := model.User{}
	_ = mapstructure.Decode(req, &userModel)
	userModel.Password = string(hashedPassword)
	err = t.UserRepository.Update(userModel)
	return err
}

// Delete implements UserService
func (t *UserServiceImpl) Delete(id int64) error {
	err := t.UserRepository.Delete(id)
	err = t.UserTeamRepository.DeleteByUserId(id)
	return err
}

func (t *UserServiceImpl) Find(req request.UserFindRequest) (users []response.UserResponse, pageCount int64, err error) {
	result, count, err := t.UserRepository.Find(req)
	if req.Size >= 1 && req.Page >= 1 {
		pageCount = int64(math.Ceil(float64(count) / float64(req.Size)))
	} else {
		pageCount = 1
	}
	for _, value := range result {
		userResp := response.UserResponse{}
		_ = mapstructure.Decode(value, &userResp)
		userTeams, _ := t.UserTeamRepository.FindByUserId(value.UserId)
		for _, v1 := range userTeams {
			userResp.TeamIds = append(userResp.TeamIds, v1.TeamId)
		}
		users = append(users, userResp)
	}
	return users, pageCount, err
}

// FindById implements UserService
func (t *UserServiceImpl) FindById(id int64) (response.UserResponse, error) {
	userData, err := t.UserRepository.FindById(id)
	if err != nil {
		return response.UserResponse{}, errors.New("用户不存在")
	}
	userResp := response.UserResponse{}
	_ = mapstructure.Decode(userData, &userResp)
	userTeams, _ := t.UserTeamRepository.FindByUserId(id)
	for _, value := range userTeams {
		userResp.TeamIds = append(userResp.TeamIds, value.TeamId)
	}
	return userResp, nil
}

func (t *UserServiceImpl) FindByUsername(username string) (response.UserResponse, error) {
	userData, err := t.UserRepository.FindByUsername(username)
	if err != nil {
		return response.UserResponse{}, errors.New("用户不存在")
	}
	userResp := response.UserResponse{}
	_ = mapstructure.Decode(userData, &userResp)
	userTeams, _ := t.UserTeamRepository.FindByUserId(userResp.UserId)
	for _, value := range userTeams {
		userResp.TeamIds = append(userResp.TeamIds, value.TeamId)
	}
	return userResp, nil
}

func (t *UserServiceImpl) FindByEmail(email string) (user response.UserResponse, err error) {
	userData, err := t.UserRepository.FindByEmail(email)
	if err != nil {
		return user, errors.New("用户不存在")
	}
	_ = mapstructure.Decode(userData, &user)
	userTeams, _ := t.UserTeamRepository.FindByUserId(user.UserId)
	for _, value := range userTeams {
		user.TeamIds = append(user.TeamIds, value.TeamId)
	}
	return user, err
}

func (t *UserServiceImpl) VerifyPasswordById(id int64, password string) bool {
	userData, err := t.UserRepository.FindById(id)
	err = bcrypt.CompareHashAndPassword([]byte(userData.Password), []byte(password))
	if err != nil {
		return false
	}
	return true
}

func (t *UserServiceImpl) VerifyPasswordByUsername(username string, password string) bool {
	userData, err := t.UserRepository.FindByUsername(username)
	err = bcrypt.CompareHashAndPassword([]byte(userData.Password), []byte(password))
	if err != nil {
		return false
	}
	return true
}
