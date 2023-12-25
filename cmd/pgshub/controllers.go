package main

import (
	"github.com/elabosak233/pgshub/internal/controllers"
	"github.com/elabosak233/pgshub/internal/services"
)

func InitControllers(appService *services.AppService) *controllers.AppController {
	return &controllers.AppController{
		UserController:      controllers.NewUserControllerImpl(appService),
		ChallengeController: controllers.NewChallengeController(appService),
		InstanceController:  controllers.NewInstanceControllerImpl(appService),
		ConfigController:    controllers.NewConfigControllerImpl(appService),
		AssetController:     controllers.NewAssetControllerImpl(appService),
		TeamController:      controllers.NewTeamControllerImpl(appService),
	}
}
