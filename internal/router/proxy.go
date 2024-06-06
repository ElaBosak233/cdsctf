package router

import (
	"github.com/elabosak233/cloudsdale/internal/controller"
	"github.com/gin-gonic/gin"
)

type IProxyRouter interface {
	Register()
}

type ProxyRouter struct {
	router     *gin.RouterGroup
	controller controller.IProxyController
}

func NewProxyRouter(proxyRouter *gin.RouterGroup, proxyController controller.IProxyController) IProxyRouter {
	return &ProxyRouter{
		router:     proxyRouter,
		controller: proxyController,
	}
}

func (p *ProxyRouter) Register() {
	p.router.Any("/:id", p.controller.Connect)
}
