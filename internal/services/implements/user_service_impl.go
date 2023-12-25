package implements

import (
	"errors"
	"github.com/dgrijalva/jwt-go"
	model "github.com/elabosak233/pgshub/internal/models/data"
	"github.com/elabosak233/pgshub/internal/models/misc"
	"github.com/elabosak233/pgshub/internal/models/request"
	"github.com/elabosak233/pgshub/internal/models/response"
	"github.com/elabosak233/pgshub/internal/repositorys"
	"github.com/elabosak233/pgshub/internal/repositorys/relations"
	"github.com/elabosak233/pgshub/internal/services"
	"github.com/google/uuid"
	"github.com/mitchellh/mapstructure"
	"github.com/spf13/viper"
	"golang.org/x/crypto/bcrypt"
	"time"
)

type UserServiceImpl struct {
	UserRepository     repositorys.UserRepository
	UserTeamRepository relations.UserTeamRepository
}

func NewUserServiceImpl(appRepository *repositorys.AppRepository) services.UserService {
	return &UserServiceImpl{
		UserRepository:     appRepository.UserRepository,
		UserTeamRepository: appRepository.UserTeamRepository,
	}
}

func (t *UserServiceImpl) GetJwtTokenById(user response.UserResponse) (tokenString string, err error) {
	expiration := time.Now().Add(time.Duration(viper.GetInt("jwt.expiration")) * time.Minute)
	jwtSecretKey := []byte(viper.GetString("jwt.secret_key"))
	claims := &misc.Claims{
		UserId: user.UserId,
		StandardClaims: jwt.StandardClaims{
			ExpiresAt: expiration.Unix(),
		},
	}
	token := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)
	tokenString, err = token.SignedString(jwtSecretKey)
	return tokenString, err
}

func (t *UserServiceImpl) GetIdByJwtToken(token string) (id string, err error) {
	pgsToken, err := jwt.ParseWithClaims(token, &misc.Claims{}, func(token *jwt.Token) (interface{}, error) {
		return []byte(viper.GetString("jwt.secret_key")), nil
	})
	if err != nil {
		return "", err
	}
	if claims, ok := pgsToken.Claims.(*misc.Claims); ok && pgsToken.Valid {
		return claims.UserId, nil
	} else {
		return "", errors.New("无效 Token")
	}
}

// Create implements UserService
func (t *UserServiceImpl) Create(req model.User) error {
	hashedPassword, _ := bcrypt.GenerateFromPassword([]byte(req.Password), bcrypt.DefaultCost)
	userModel := model.User{
		UserId:   uuid.NewString(),
		Username: req.Username,
		Email:    req.Email,
		Role:     3,
		Password: string(hashedPassword),
	}
	err := t.UserRepository.Insert(userModel)
	return err
}

// Update implements UserService
func (t *UserServiceImpl) Update(req request.UserUpdateRequest) error {
	hashedPassword, _ := bcrypt.GenerateFromPassword([]byte(req.Password), bcrypt.DefaultCost)
	userData, err := t.UserRepository.FindById(req.UserId)
	if err != nil || userData.UserId == "" {
		return errors.New("用户不存在")
	}
	userModel := model.User{}
	_ = mapstructure.Decode(req, &userModel)
	userModel.Password = string(hashedPassword)
	err = t.UserRepository.Update(userModel)
	return err
}

// Delete implements UserService
func (t *UserServiceImpl) Delete(id string) error {
	err := t.UserRepository.Delete(id)
	err = t.UserTeamRepository.DeleteByUserId(id)
	return err
}

// FindAll implements UserService
func (t *UserServiceImpl) FindAll() ([]response.UserResponse, error) {
	result, _ := t.UserRepository.FindAll()
	var users []response.UserResponse
	for _, value := range result {
		userResp := response.UserResponse{}
		_ = mapstructure.Decode(value, &userResp)
		userTeams, _ := t.UserTeamRepository.FindByUserId(value.UserId)
		for _, v1 := range userTeams {
			userResp.TeamIds = append(userResp.TeamIds, v1.TeamId)
		}
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

func (t *UserServiceImpl) VerifyPasswordById(id string, password string) bool {
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
