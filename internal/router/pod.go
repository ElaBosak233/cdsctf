package router

import (
	"github.com/elabosak233/pgshub/internal/controller"
	"github.com/elabosak233/pgshub/internal/middleware"
	"github.com/gin-gonic/gin"
)

func NewPodRouter(podRouter *gin.RouterGroup, podController controller.IPodController, authMiddleware middleware.IAuthMiddleware) {
	podRouter.GET("/", authMiddleware.Auth(), podController.Find)
	podRouter.GET("/:id", podController.FindById)
	podRouter.POST("/", authMiddleware.Auth(), podController.Create)
	podRouter.DELETE("/", authMiddleware.Auth(), podController.Remove)
	podRouter.PUT("/", authMiddleware.Auth(), podController.Renew)
}
