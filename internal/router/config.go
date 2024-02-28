package router

import (
	"github.com/elabosak233/cloudsdale/internal/controller"
	"github.com/gin-gonic/gin"
)

func NewConfigRouter(configRouter *gin.RouterGroup, configController controller.IConfigController) {
	configRouter.GET("/", configController.Find)
	configRouter.PUT("/", configController.Update)
}
