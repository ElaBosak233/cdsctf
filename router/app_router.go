package router

import (
	"github.com/elabosak233/pgshub/controller"
	"github.com/elabosak233/pgshub/utils"
	"github.com/gin-gonic/gin"
	"net/http"
)

func NewRouters(
	router *gin.Engine,
	userController *controller.UserController,
	groupController *controller.GroupController,
	challengeController *controller.ChallengeController,
) {
	router.GET("/", func(ctx *gin.Context) {
		ctx.String(http.StatusOK, utils.GetInfo())
	})
	userRouter := router.Group("/user")
	NewUserRouter(userRouter, userController)
	groupRouter := router.Group("/group")
	NewGroupRouter(groupRouter, groupController)
	challengeRouter := router.Group("/challenge")
	NewChallengeRouter(challengeRouter, challengeController)
}
