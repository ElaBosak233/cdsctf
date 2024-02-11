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
	return &Controllers{
		UserController:       NewUserControllerImpl(appService),
		ChallengeController:  NewChallengeController(appService),
		InstanceController:   NewInstanceControllerImpl(appService),
		ConfigController:     NewConfigControllerImpl(appService),
		MediaController:      NewMediaControllerImpl(appService),
		TeamController:       NewTeamControllerImpl(appService),
		SubmissionController: NewSubmissionControllerImpl(appService),
		GameController:       NewGameControllerImpl(appService),
	}
}
