package routers

import (
	"github.com/elabosak233/pgshub/internal/controllers"
	"github.com/elabosak233/pgshub/internal/middlewares"
	"github.com/gin-gonic/gin"
)

func NewInstanceRouter(instanceRouter *gin.RouterGroup, instanceController controllers.InstanceController, authMiddleware middlewares.AuthMiddleware) {
	instanceRouter.GET("/", instanceController.Find)
	instanceRouter.GET("/:id", instanceController.FindById)
	instanceRouter.POST("/", authMiddleware.Auth(), instanceController.Create)
	instanceRouter.DELETE("/", instanceController.Remove)
	instanceRouter.PUT("/", instanceController.Renew)
}
