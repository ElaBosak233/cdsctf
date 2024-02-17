package router

import (
	"github.com/elabosak233/cloudsdale/internal/controller"
	"github.com/elabosak233/cloudsdale/internal/middleware"
	"github.com/gin-gonic/gin"
)

func NewConfigRouter(configRouter *gin.RouterGroup, configController controller.IConfigController, authMiddleware middleware.IAuthMiddleware) {
	configRouter.GET("/", configController.Find)
	configRouter.PUT("/", authMiddleware.AuthInRole(1), configController.Update)
}
