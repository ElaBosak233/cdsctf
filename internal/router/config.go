package router

import (
	"github.com/elabosak233/cloudsdale/internal/controller"
	"github.com/gin-gonic/gin"
)

type IConfigRouter interface {
	Register()
}

type ConfigRouter struct {
	router     *gin.RouterGroup
	controller controller.IConfigController
}

func NewConfigRouter(configRouter *gin.RouterGroup, configController controller.IConfigController) IConfigRouter {
	return &ConfigRouter{
		router:     configRouter,
		controller: configController,
	}
}

func (c *ConfigRouter) Register() {
	c.router.GET("/", c.controller.Find)
	c.router.PUT("/", c.controller.Update)
}
