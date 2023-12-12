package main

import (
	"github.com/elabosak233/pgshub/controller"
	"github.com/elabosak233/pgshub/service"
)

func InitControllers(appService *service.AppService) *controller.AppController {
	return &controller.AppController{
		UserController:      controller.NewUserControllerImpl(appService),
		GroupController:     controller.NewGroupControllerImpl(appService),
		ChallengeController: controller.NewChallengeController(appService),
		InstanceController:  controller.NewInstanceControllerImpl(appService),
		ConfigController:    controller.NewConfigControllerImpl(appService),
		AssetController:     controller.NewAssetControllerImpl(appService),
		TeamController:      controller.NewTeamControllerImpl(appService),
	}
}
