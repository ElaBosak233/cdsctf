package controllers

import "github.com/elabosak233/pgshub/services"

type Controllers struct {
	UserController       UserController
	ChallengeController  ChallengeController
	InstanceController   PodController
	ConfigController     ConfigController
	MediaController      MediaController
	TeamController       TeamController
	SubmissionController SubmissionController
	GameController       GameController
}

func InitControllers(appService *services.Services) *Controllers {

	userController := NewUserControllerImpl(appService)
	challengeController := NewChallengeController(appService)
	instanceController := NewInstanceControllerImpl(appService)
	configController := NewConfigControllerImpl(appService)
	mediaController := NewMediaControllerImpl(appService)
	teamController := NewTeamControllerImpl(appService)
	submissionController := NewSubmissionControllerImpl(appService)
	gameController := NewGameControllerImpl(appService)

	return &Controllers{
		UserController:       userController,
		ChallengeController:  challengeController,
		InstanceController:   instanceController,
		ConfigController:     configController,
		MediaController:      mediaController,
		TeamController:       teamController,
		SubmissionController: submissionController,
		GameController:       gameController,
	}
}
