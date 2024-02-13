package router

import (
	"github.com/elabosak233/pgshub/internal/controller"
	"github.com/elabosak233/pgshub/internal/middleware"
	"github.com/gin-gonic/gin"
)

func NewConfigRouter(configRouter *gin.RouterGroup, configController controller.IConfigController, authMiddleware middleware.IAuthMiddleware) {
	configRouter.GET("/", configController.Find)
	configRouter.PUT("/", authMiddleware.AuthInRole(1), configController.Update)
}
