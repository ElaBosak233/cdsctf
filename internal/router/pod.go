package router

import (
	"github.com/elabosak233/cloudsdale/internal/controller"
	"github.com/gin-gonic/gin"
)

type IPodRouter interface {
	Register()
}

type PodRouter struct {
	router     *gin.RouterGroup
	controller controller.IPodController
}

func NewPodRouter(podRouter *gin.RouterGroup, podController controller.IPodController) IPodRouter {
	return &PodRouter{
		router:     podRouter,
		controller: podController,
	}
}

func (p *PodRouter) Register() {
	p.router.GET("/", p.controller.Find)
	p.router.POST("/", p.controller.Create)
	p.router.DELETE("/:id", p.controller.Remove)
	p.router.PUT("/:id", p.controller.Renew)
}
