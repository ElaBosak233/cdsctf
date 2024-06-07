package controller

import (
	"github.com/elabosak233/cloudsdale/internal/service"
	"go.uber.org/zap"
	"sync"
)

var (
	c              *Controller = nil
	onceController sync.Once
)

type Controller struct {
	UserController       IUserController
	ChallengeController  IChallengeController
	PodController        IPodController
	ConfigController     IConfigController
	MediaController      IMediaController
	TeamController       ITeamController
	SubmissionController ISubmissionController
	GameController       IGameController
	CategoryController   ICategoryController
	ProxyController      IProxyController
}

func C() *Controller {
	if c == nil {
		InitController()
	}
	return c
}

func InitController() {
	onceController.Do(func() {
		s := service.S()

		c = &Controller{
			UserController:       NewUserController(s),
			ChallengeController:  NewChallengeController(s),
			PodController:        NewPodController(s),
			ConfigController:     NewConfigController(s),
			MediaController:      NewMediaController(s),
			TeamController:       NewTeamController(s),
			SubmissionController: NewSubmissionController(s),
			GameController:       NewGameController(s),
			CategoryController:   NewCategoryController(s),
			ProxyController:      NewProxyController(),
		}
	})
	zap.L().Info("Controller layer inits successfully.")
}
