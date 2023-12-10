package router

import (
	"github.com/elabosak233/pgshub/controller"
	"github.com/gin-gonic/gin"
)

func NewUserRouter(userRouter *gin.RouterGroup, userController controller.UserController) {
	userRouter.GET("/", userController.FindAll)
	userRouter.POST("/", userController.Create)
	userRouter.POST("/register", userController.Register)
	userRouter.PUT("/", userController.Update)
	userRouter.DELETE("/", userController.Delete)
	userRouter.GET("/id/:id", userController.FindById)
	userRouter.GET("/username/:username", userController.FindByUsername)
	userRouter.POST("/login", userController.Login)
	userRouter.POST("/logout", userController.Logout)
	userRouter.GET("/token/:token", userController.VerifyToken)
}
