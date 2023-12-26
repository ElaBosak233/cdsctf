package routers

import (
	"github.com/elabosak233/pgshub/internal/controllers"
	"github.com/gin-gonic/gin"
)

func NewConfigRouter(configRouter *gin.RouterGroup, configController controllers.ConfigController) {
	configRouter.GET("/", configController.Find)
}
