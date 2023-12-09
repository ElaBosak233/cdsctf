package router

import (
	"github.com/elabosak233/pgshub/controller"
	"github.com/gin-gonic/gin"
)

func NewUserRouter(userRouter *gin.RouterGroup, userController controller.UserController) {
	userRouter.GET("/", userController.FindAll)
	userRouter.POST("/", userController.Create)
	userRouter.POST("/register", userController.Register)
	userRouter.PATCH("/", userController.Update)
	userRouter.GET("/id/:id", userController.FindById)
	userRouter.GET("/username/:username", userController.FindByUsername)
	userRouter.DELETE("/:id", userController.Delete)
	userRouter.POST("/login", userController.Login)
	userRouter.POST("/logout", userController.Logout)
	userRouter.GET("/verifyToken/:token", userController.VerifyToken)
}
