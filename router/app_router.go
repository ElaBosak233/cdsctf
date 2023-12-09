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
	userRouter := router.Group("/user")
	NewUserRouter(userRouter, appController.UserController)
	groupRouter := router.Group("/group")
	NewGroupRouter(groupRouter, appController.GroupController, appController.UserGroupController)
	challengeRouter := router.Group("/challenge")
	NewChallengeRouter(challengeRouter, appController.ChallengeController)
	instanceRouter := router.Group("/instance")
	NewInstanceRouter(instanceRouter, appController.InstanceController)
}
