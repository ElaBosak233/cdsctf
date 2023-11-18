package router

import (
	controller2 "github.com/elabosak233/pgshub/controller"
	"github.com/elabosak233/pgshub/utils"
	"github.com/gin-gonic/gin"
	"net/http"
)

func NewRouters(
	userController *controller2.UserController,
	groupController *controller2.GroupController,
) *gin.Engine {
	router := gin.Default()
	router.GET("/", func(ctx *gin.Context) {
		ctx.String(http.StatusOK, utils.GetInfo())
	})
	userRouter := router.Group("/user")
	NewUserRouter(userRouter, userController)
	groupRouter := router.Group("/group")
	NewGroupRouter(groupRouter, groupController)
	return router
}
