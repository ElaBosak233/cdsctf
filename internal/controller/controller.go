package controller

import (
	"github.com/elabosak233/cloudsdale/internal/service"
	"sync"
)

var (
	c              *Controller = nil
	onceController sync.Once
)

type Controller struct {
	UserController       IUserController
	ChallengeController  IChallengeController
	InstanceController   IPodController
	ConfigController     IConfigController
	MediaController      IMediaController
	TeamController       ITeamController
	SubmissionController ISubmissionController
	GameController       IGameController
	CategoryController   ICategoryController
	ProxyController      IProxyController
	GroupController      IGroupController
}

func C() *Controller {
	return c
}

func InitController() {
	onceController.Do(func() {
		appService := service.S()

		userController := NewUserController(appService)
		challengeController := NewChallengeController(appService)
		instanceController := NewPodController(appService)
		configController := NewConfigController(appService)
		mediaController := NewMediaController(appService)
		teamController := NewTeamController(appService)
		submissionController := NewSubmissionController(appService)
		gameController := NewGameController(appService)
		categoryController := NewCategoryController(appService)
		proxyController := NewProxyController()
		groupController := NewGroupController(appService)

		c = &Controller{
			UserController:       userController,
			ChallengeController:  challengeController,
			InstanceController:   instanceController,
			ConfigController:     configController,
			MediaController:      mediaController,
			TeamController:       teamController,
			SubmissionController: submissionController,
			GameController:       gameController,
			CategoryController:   categoryController,
			ProxyController:      proxyController,
			GroupController:      groupController,
		}
	})
}
