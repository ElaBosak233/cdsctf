package router

import (
	"github.com/elabosak233/pgshub/internal/controller"
	"github.com/gin-gonic/gin"
	"net/http"
)

func NewRouters(
	userController *controller.UserController,
	groupController *controller.GroupController,
) *gin.Engine {
	router := gin.Default()
	router.GET("", func(ctx *gin.Context) {
		ctx.JSON(http.StatusOK, "welcome home")
	})
	userRouter := router.Group("/user")
	NewUserRouter(userRouter, userController)
	groupRouter := router.Group("/group")
	NewGroupRouter(groupRouter, groupController)
	return router
}
