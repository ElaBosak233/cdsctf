package routers

import (
	"github.com/elabosak233/pgshub/controllers"
	"github.com/elabosak233/pgshub/middlewares"
	"github.com/gin-gonic/gin"
)

func NewUserRouter(userRouter *gin.RouterGroup, userController controllers.UserController, authMiddleware middlewares.AuthMiddleware) {
	userRouter.GET("/", userController.Find)
	userRouter.POST("/", authMiddleware.AuthInRole(1), userController.Create)
	userRouter.POST("/register", userController.Register)
	userRouter.PUT("/", authMiddleware.Auth(), userController.Update)
	userRouter.DELETE("/", authMiddleware.Auth(), userController.Delete)
	userRouter.POST("/login", userController.Login)
	userRouter.POST("/logout", authMiddleware.Auth(), userController.Logout)
	userRouter.GET("/token/:token", userController.VerifyToken)
}
