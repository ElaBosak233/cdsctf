package router

import (
	"github.com/elabosak233/cloudsdale/internal/controller"
	"github.com/gin-gonic/gin"
)

func NewProxyRouter(proxyRouter *gin.RouterGroup, proxyController controller.IProxyController) {
	proxyRouter.GET("/:id", proxyController.Connect)
}
