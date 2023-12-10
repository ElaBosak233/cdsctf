package router

import (
	"fmt"
	"github.com/elabosak233/pgshub/controller"
	"github.com/gin-gonic/gin"
	"net/http"
)

func NewRouters(
	router *gin.RouterGroup,
	appController *controller.AppController,
) {
	router.GET("/", func(ctx *gin.Context) {
		ctx.String(http.StatusOK, fmt.Sprintf("The Backend of PgsHub"))
	})
	userRouter := router.Group("/users")
	NewUserRouter(userRouter, appController.UserController)
	groupRouter := router.Group("/groups")
	NewGroupRouter(groupRouter, appController.GroupController, appController.UserGroupController)
	challengeRouter := router.Group("/challenges")
	NewChallengeRouter(challengeRouter, appController.ChallengeController)
	instanceRouter := router.Group("/instances")
	NewInstanceRouter(instanceRouter, appController.InstanceController)
	configRouter := router.Group("/configs")
	NewConfigRouter(configRouter, appController.ConfigController)
}
