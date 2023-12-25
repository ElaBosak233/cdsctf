package main

import (
	"github.com/elabosak233/pgshub/internal/controllers"
	"github.com/elabosak233/pgshub/internal/controllers/implements"
	"github.com/elabosak233/pgshub/internal/services"
)

func InitControllers(appService *services.AppService) *controllers.AppController {
	return &controllers.AppController{
		UserController:      implements.NewUserControllerImpl(appService),
		ChallengeController: implements.NewChallengeController(appService),
		InstanceController:  implements.NewInstanceControllerImpl(appService),
		ConfigController:    implements.NewConfigControllerImpl(appService),
		AssetController:     implements.NewAssetControllerImpl(appService),
		TeamController:      implements.NewTeamControllerImpl(appService),
	}
}
