package router

import (
	"github.com/elabosak233/pgshub/controller"
	"github.com/gin-gonic/gin"
)

func NewInstanceRouter(instanceRouter *gin.RouterGroup, instanceController controller.InstanceController) {
	instanceRouter.GET("/", instanceController.FindAll)
	instanceRouter.GET("/:id", instanceController.FindById)
	instanceRouter.POST("/", instanceController.Create)
	instanceRouter.DELETE("/", instanceController.Remove)
	instanceRouter.GET("/renew", instanceController.Renew)
}
