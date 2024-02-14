package service

import (
	"errors"
	"github.com/elabosak233/pgshub/internal/config"
	"github.com/elabosak233/pgshub/internal/model"
	"github.com/elabosak233/pgshub/internal/model/dto/request"
	"github.com/elabosak233/pgshub/internal/model/dto/response"
	"github.com/elabosak233/pgshub/internal/repository"
	"github.com/golang-jwt/jwt/v5"
	"github.com/mitchellh/mapstructure"
	"golang.org/x/crypto/bcrypt"
	"math"
	"time"
)

type IUserService interface {
	Create(req request.UserCreateRequest) (err error)
	Update(req request.UserUpdateRequest) (err error)
	Delete(id int64) error
	FindById(id int64) (response.UserResponse, error)
	FindByUsername(username string) (response.UserResponse, error)
	FindByEmail(email string) (user response.UserResponse, err error)
	VerifyPasswordById(id int64, password string) bool
	VerifyPasswordByUsername(username string, password string) bool
	GetJwtTokenById(user response.UserResponse) (tokenString string, err error)
	GetIdByJwtToken(token string) (id int64, err error)
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

func (t *UserService) GetIdByJwtToken(token string) (id int64, err error) {
	pgsToken, err := jwt.Parse(token, func(token *jwt.Token) (interface{}, error) {
		return []byte(config.AppCfg().Jwt.SecretKey), nil
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

func (t *UserService) Create(req request.UserCreateRequest) (err error) {
	hashedPassword, _ := bcrypt.GenerateFromPassword([]byte(req.Password), bcrypt.DefaultCost)
	userModel := model.User{
		Username: req.Username,
		Email:    req.Email,
		Nickname: req.Nickname,
		Role:     req.Role,
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

func (t *UserService) Delete(id int64) error {
	err := t.UserRepository.Delete(id)
	err = t.UserTeamRepository.DeleteByUserId(id)
	return err
}

func (t *UserService) Find(req request.UserFindRequest) (users []response.UserResponse, pageCount int64, total int64, err error) {
	users, count, err := t.UserRepository.Find(req)
	var userIds []int64
	usersMap := make(map[int64]response.UserResponse)
	for _, result := range users {
		if _, ok := usersMap[result.ID]; !ok {
			usersMap[result.ID] = result
		}
		userIds = append(userIds, result.ID)
	}
	teams, err := t.TeamRepository.BatchFindByUserId(request.TeamBatchFindByUserIdRequest{
		UserID: userIds,
	})
	for _, team := range teams {
		var teamResponse response.TeamSimpleResponse
		_ = mapstructure.Decode(team, &teamResponse)
		if user, ok := usersMap[team.UserId]; ok {
			user.Teams = append(user.Teams, teamResponse)
			usersMap[team.UserId] = user
		}
	}
	for index, user := range users {
		users[index].Teams = usersMap[user.ID].Teams
	}
	if req.Size >= 1 && req.Page >= 1 {
		pageCount = int64(math.Ceil(float64(count) / float64(req.Size)))
	} else {
		pageCount = 1
	}
	return users, pageCount, count, err
}

func (t *UserService) FindById(id int64) (response.UserResponse, error) {
	userData, err := t.UserRepository.FindById(id)
	if err != nil {
		return response.UserResponse{}, errors.New("用户不存在")
	}
	userResp := response.UserResponse{}
	_ = mapstructure.Decode(userData, &userResp)
	return userResp, nil
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

func (t *UserService) VerifyPasswordById(id int64, password string) bool {
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
