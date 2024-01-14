package routers

import (
	"github.com/elabosak233/pgshub/internal/controllers"
	"github.com/elabosak233/pgshub/internal/middlewares"
	"github.com/gin-gonic/gin"
)

func NewConfigRouter(configRouter *gin.RouterGroup, configController controllers.ConfigController, authMiddleware middlewares.AuthMiddleware) {
	configRouter.GET("/", configController.Find)
	configRouter.PUT("/", authMiddleware.AuthInRole(1), configController.Update)
}
