package controller

import "github.com/elabosak233/cloudsdale/internal/service"

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
}

func InitController(appService *service.Service) *Controller {

	userController := NewUserController(appService)
	challengeController := NewChallengeController(appService)
	instanceController := NewInstanceController(appService)
	configController := NewConfigController(appService)
	mediaController := NewMediaController(appService)
	teamController := NewTeamController(appService)
	submissionController := NewSubmissionController(appService)
	gameController := NewGameController(appService)
	categoryController := NewCategoryController(appService)
	proxyController := NewProxyController()

	return &Controller{
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
	}
}
