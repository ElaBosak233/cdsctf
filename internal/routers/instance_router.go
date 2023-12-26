package routers

import (
	"github.com/elabosak233/pgshub/internal/controllers"
	"github.com/gin-gonic/gin"
)

func NewInstanceRouter(instanceRouter *gin.RouterGroup, instanceController controllers.InstanceController) {
	instanceRouter.GET("/", instanceController.Find)
	instanceRouter.GET("/:id", instanceController.FindById)
	instanceRouter.POST("/", instanceController.Create)
	instanceRouter.DELETE("/", instanceController.Remove)
	instanceRouter.PUT("/", instanceController.Renew)
}
