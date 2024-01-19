package routers

import (
	"github.com/elabosak233/pgshub/controllers"
	"github.com/elabosak233/pgshub/middlewares"
	"github.com/gin-gonic/gin"
)

func NewInstanceRouter(instanceRouter *gin.RouterGroup, instanceController controllers.InstanceController, authMiddleware middlewares.AuthMiddleware) {
	instanceRouter.GET("/", authMiddleware.Auth(), instanceController.Find)
	instanceRouter.GET("/:id", instanceController.FindById)
	instanceRouter.POST("/", authMiddleware.Auth(), instanceController.Create)
	instanceRouter.DELETE("/", authMiddleware.Auth(), instanceController.Remove)
	instanceRouter.PUT("/", authMiddleware.Auth(), instanceController.Renew)
}
