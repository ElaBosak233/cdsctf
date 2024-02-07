package controllers

import "github.com/elabosak233/pgshub/services"

type Controllers struct {
	UserController       UserController
	ChallengeController  ChallengeController
	InstanceController   PodController
	ConfigController     ConfigController
	AssetController      AssetController
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
		AssetController:      NewAssetControllerImpl(appService),
		TeamController:       NewTeamControllerImpl(appService),
		SubmissionController: NewSubmissionControllerImpl(appService),
		GameController:       NewGameControllerImpl(appService),
	}
}
