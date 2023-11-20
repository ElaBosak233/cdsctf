package router

import (
	"fmt"
	"github.com/elabosak233/pgshub/controller"
	"github.com/gin-gonic/gin"
	"net/http"
)

func NewRouters(
	router *gin.Engine,
	userController *controller.UserController,
	groupController *controller.GroupController,
	challengeController *controller.ChallengeController,
	userGroupController *controller.UserGroupController,
) {
	router.GET("/", func(ctx *gin.Context) {
		ctx.String(http.StatusOK, fmt.Sprintf("The Backend of PgsHub"))
	})
	userRouter := router.Group("/user")
	NewUserRouter(userRouter, userController)
	groupRouter := router.Group("/group")
	NewGroupRouter(groupRouter, groupController, userGroupController)
	challengeRouter := router.Group("/challenge")
	NewChallengeRouter(challengeRouter, challengeController)
}
