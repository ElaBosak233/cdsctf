package router

import (
	"github.com/elabosak233/cloudsdale/internal/controller"
	"github.com/gin-gonic/gin"
)

func NewPodRouter(podRouter *gin.RouterGroup, podController controller.IPodController) {
	podRouter.GET("/", podController.Find)
	podRouter.GET("/:id", podController.FindById)
	podRouter.POST("/", podController.Create)
	podRouter.DELETE("/:id", podController.Remove)
	podRouter.PUT("/:id", podController.Renew)
}
