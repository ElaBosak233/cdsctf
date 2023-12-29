package initialize

import (
	"github.com/elabosak233/pgshub/internal/controllers"
	registry2 "github.com/elabosak233/pgshub/internal/services"
)

func Controllers(appService *registry2.AppService) *controllers.AppController {
	return &controllers.AppController{
		UserController:       controllers.NewUserControllerImpl(appService),
		ChallengeController:  controllers.NewChallengeController(appService),
		InstanceController:   controllers.NewInstanceControllerImpl(appService),
		ConfigController:     controllers.NewConfigControllerImpl(appService),
		AssetController:      controllers.NewAssetControllerImpl(appService),
		TeamController:       controllers.NewTeamControllerImpl(appService),
		SubmissionController: controllers.NewSubmissionControllerImpl(appService),
	}
}
