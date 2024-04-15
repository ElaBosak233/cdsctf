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
	return c
}

func InitController() {
	onceController.Do(func() {
		appService := service.S()

		c = &Controller{
			UserController:       NewUserController(appService),
			ChallengeController:  NewChallengeController(appService),
			PodController:        NewPodController(appService),
			ConfigController:     NewConfigController(appService),
			MediaController:      NewMediaController(appService),
			TeamController:       NewTeamController(appService),
			SubmissionController: NewSubmissionController(appService),
			GameController:       NewGameController(appService),
			CategoryController:   NewCategoryController(appService),
			ProxyController:      NewProxyController(),
		}
	})
	zap.L().Info("Controller layer inits successfully.")
}
