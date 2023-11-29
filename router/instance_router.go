package router

import (
	"github.com/elabosak233/pgshub/controller"
	"github.com/gin-gonic/gin"
)

func NewInstanceRouter(instanceRouter *gin.RouterGroup, instanceController controller.InstanceController) {
	instanceRouter.POST("/create", instanceController.Create)
	instanceRouter.GET("/status", instanceController.Status)
	instanceRouter.GET("/remove", instanceController.Remove)
	instanceRouter.GET("/renew", instanceController.Renew)
}
