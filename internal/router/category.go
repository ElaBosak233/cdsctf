package router

import (
	"github.com/elabosak233/cloudsdale/internal/controller"
	"github.com/gin-gonic/gin"
)

type ICategoryRouter interface {
	Register()
}

type CategoryRouter struct {
	router     *gin.RouterGroup
	controller controller.ICategoryController
}

func NewCategoryRouter(categoryRouter *gin.RouterGroup, categoryController controller.ICategoryController) ICategoryRouter {
	return &CategoryRouter{
		router:     categoryRouter,
		controller: categoryController,
	}
}

func (c *CategoryRouter) Register() {
	c.router.POST("/", c.controller.Create)
	c.router.PUT("/:id", c.controller.Update)
	c.router.GET("/", c.controller.Find)
}
