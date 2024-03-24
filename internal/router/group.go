package router

import (
	"github.com/elabosak233/cloudsdale/internal/controller"
	"github.com/gin-gonic/gin"
)

type IGroupRouter interface {
	Register()
}

type GroupRouter struct {
	router     *gin.RouterGroup
	controller controller.IGroupController
}

func NewGroupRouter(groupRouter *gin.RouterGroup, groupController controller.IGroupController) IGroupRouter {
	return &GroupRouter{
		router:     groupRouter,
		controller: groupController,
	}
}

func (g *GroupRouter) Register() {
	g.router.GET("/", g.controller.Find)
	g.router.PUT("/:id", g.controller.Update)
}
