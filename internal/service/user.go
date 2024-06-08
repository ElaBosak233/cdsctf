package service

import (
	"errors"
	"github.com/elabosak233/cloudsdale/internal/app/config"
	"github.com/elabosak233/cloudsdale/internal/extension/captcha"
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"github.com/elabosak233/cloudsdale/internal/repository"
	"github.com/golang-jwt/jwt/v5"
	"github.com/mitchellh/mapstructure"
	"golang.org/x/crypto/bcrypt"
	"strings"
	"time"
)

type IUserService interface {
	// Create will create a new user with the given request.
	Create(req request.UserCreateRequest) error

	// Update will update the user with the given request.
	Update(req request.UserUpdateRequest) error

	// Delete will delete the user with the given id.
	Delete(id uint) error

	// Find will return the users, total count and error.
	Find(req request.UserFindRequest) ([]model.User, int64, error)

	// Register will create a new user with the given request, but the default group is user.
	Register(req request.UserRegisterRequest) error

	// Login will verify the user login request and return the user and jwt token.
	Login(req request.UserLoginRequest) (model.User, string, error)

	// Logout will log out the user with the given token.
	Logout(token string) (uint, error)
}

type UserService struct {
	userRepository     repository.IUserRepository
	teamRepository     repository.ITeamRepository
	userTeamRepository repository.IUserTeamRepository
}

func NewUserService(r *repository.Repository) IUserService {
	return &UserService{
		userRepository:     r.UserRepository,
		teamRepository:     r.TeamRepository,
		userTeamRepository: r.UserTeamRepository,
	}
}

func (t *UserService) GetJwtTokenByID(user model.User) (tokenString string, err error) {
	jwtSecretKey := []byte(config.JwtSecretKey())
	pgsToken := jwt.NewWithClaims(jwt.SigningMethodHS256, jwt.MapClaims{
		"user_id": user.ID,
		"exp":     time.Now().Add(time.Duration(config.AppCfg().Gin.Jwt.Expiration) * time.Minute).Unix(),
	})
	return pgsToken.SignedString(jwtSecretKey)
}

func (t *UserService) Logout(token string) (uint, error) {
	pgsToken, err := jwt.Parse(token, func(token *jwt.Token) (interface{}, error) {
		return []byte(config.JwtSecretKey()), nil
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

func (t *UserService) Create(req request.UserCreateRequest) error {
	hashedPassword, _ := bcrypt.GenerateFromPassword([]byte(req.Password), bcrypt.DefaultCost)
	userModel := model.User{
		Username: strings.ToLower(req.Username),
		Email:    strings.ToLower(req.Email),
		Nickname: req.Nickname,
		Group:    req.Group,
		Password: string(hashedPassword),
	}
	err := t.userRepository.Create(userModel)
	return err
}

func (t *UserService) Register(req request.UserRegisterRequest) error {
	var err error
	hashedPassword, _ := bcrypt.GenerateFromPassword([]byte(req.Password), bcrypt.DefaultCost)
	success := true
	if config.PltCfg().User.Register.Captcha.Enabled {
		capt := captcha.NewCaptcha()
		success, err = capt.Verify(req.CaptchaToken, req.RemoteIP)
	}
	if success {
		userModel := model.User{
			Username: strings.ToLower(req.Username),
			Email:    strings.ToLower(req.Email),
			Nickname: req.Nickname,
			Group:    "user",
			Password: string(hashedPassword),
		}
		err = t.userRepository.Create(userModel)
	}
	return err
}

func (t *UserService) Update(req request.UserUpdateRequest) error {
	user := model.User{}
	_ = mapstructure.Decode(req, &user)
	if req.Password != "" {
		hashedPassword, _ := bcrypt.GenerateFromPassword([]byte(req.Password), bcrypt.DefaultCost)
		user.Password = string(hashedPassword)
	}
	if req.Username != "" {
		user.Username = strings.ToLower(req.Username)
	}
	if req.Email != "" {
		user.Email = strings.ToLower(req.Email)
	}
	err := t.userRepository.Update(user)
	return err
}

func (t *UserService) Delete(id uint) error {
	err := t.userRepository.Delete(id)
	return err
}

func (t *UserService) Find(req request.UserFindRequest) ([]model.User, int64, error) {
	users, total, err := t.userRepository.Find(req)
	for index := range users {
		users[index].Simplify()
	}
	return users, total, err
}

func (t *UserService) Login(req request.UserLoginRequest) (model.User, string, error) {
	user, err := t.userRepository.FindByUsername(strings.ToLower(req.Username))
	if err != nil {
		return user, "", errors.New("user.not_found")
	}
	err = bcrypt.CompareHashAndPassword([]byte(user.Password), []byte(req.Password))
	if err != nil {
		return user, "", errors.New("user.login.password_incorrect")
	}
	token, err := t.GetJwtTokenByID(user)
	return user, token, err
}
