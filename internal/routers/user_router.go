package routers

import (
	"github.com/elabosak233/pgshub/internal/controllers"
	"github.com/elabosak233/pgshub/internal/middlewares"
	"github.com/gin-gonic/gin"
)

func NewUserRouter(userRouter *gin.RouterGroup, userController controllers.UserController, authMiddleware middlewares.AuthMiddleware) {
	userRouter.GET("/", userController.FindAll)
	userRouter.POST("/", authMiddleware.Auth(), userController.Create)
	userRouter.POST("/register", userController.Register)
	userRouter.PUT("/", userController.Update)
	userRouter.DELETE("/", userController.Delete)
	userRouter.GET("/id/:id", userController.FindById)
	userRouter.GET("/username/:username", userController.FindByUsername)
	userRouter.POST("/login", userController.Login)
	userRouter.POST("/logout", userController.Logout)
	userRouter.GET("/token/:token", userController.VerifyToken)
}
