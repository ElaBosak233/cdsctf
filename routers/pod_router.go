package routers

import (
	"github.com/elabosak233/pgshub/controllers"
	"github.com/elabosak233/pgshub/middlewares"
	"github.com/gin-gonic/gin"
)

func NewPodRouter(podRouter *gin.RouterGroup, podController controllers.PodController, authMiddleware middlewares.AuthMiddleware) {
	podRouter.GET("/", authMiddleware.Auth(), podController.Find)
	podRouter.GET("/:id", podController.FindById)
	podRouter.POST("/", authMiddleware.Auth(), podController.Create)
	podRouter.DELETE("/", authMiddleware.Auth(), podController.Remove)
	podRouter.PUT("/", authMiddleware.Auth(), podController.Renew)
}
