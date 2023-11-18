package router

import (
	"github.com/elabosak233/pgshub/internal/controller"
	"github.com/gin-gonic/gin"
)

func NewUserRouter(userRouter *gin.RouterGroup, userController *controller.UserController) {
	userRouter.GET("/", userController.FindAll)
	userRouter.GET("/id/:id", userController.FindById)
	userRouter.GET("/username/:username", userController.FindByUsername)
	userRouter.POST("/", userController.Create)
	userRouter.PATCH("/:id", userController.Update)
	userRouter.DELETE("/:id", userController.Delete)
}
