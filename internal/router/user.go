package router

import (
	"github.com/elabosak233/cloudsdale/internal/controller"
	"github.com/elabosak233/cloudsdale/internal/middleware"
	"github.com/gin-gonic/gin"
)

func NewUserRouter(userRouter *gin.RouterGroup, userController controller.IUserController, authMiddleware middleware.IAuthMiddleware) {
	userRouter.GET("/", userController.Find)
	userRouter.POST("/", authMiddleware.AuthInRole(1), userController.Create)
	userRouter.POST("/register", userController.Register)
	userRouter.PUT("/", authMiddleware.Auth(), userController.Update)
	userRouter.DELETE("/", authMiddleware.Auth(), userController.Delete)
	userRouter.POST("/login", userController.Login)
	userRouter.POST("/logout", authMiddleware.Auth(), userController.Logout)
	userRouter.GET("/token/:token", userController.VerifyToken)
}
