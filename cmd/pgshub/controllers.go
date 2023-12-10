package main

import (
	"github.com/elabosak233/pgshub/controller"
	"github.com/elabosak233/pgshub/service"
)

func InitControllers(appService *service.AppService) *controller.AppController {
	return &controller.AppController{
		UserController:      controller.NewUserControllerImpl(appService),
		GroupController:     controller.NewGroupControllerImpl(appService),
		UserGroupController: controller.NewUserControllerImpl(appService),
		ChallengeController: controller.NewChallengeController(appService),
		InstanceController:  controller.NewInstanceControllerImpl(appService),
		ConfigController:    controller.NewConfigControllerImpl(appService),
	}
}
