package router

import (
	"github.com/elabosak233/cloudsdale/internal/controller"
	"github.com/gin-gonic/gin"
)

func NewCategoryRouter(categoryRouter *gin.RouterGroup, categoryController controller.ICategoryController) {
	categoryRouter.POST("/", categoryController.Create)
	categoryRouter.PUT("/:id", categoryController.Update)
	categoryRouter.GET("/", categoryController.Find)
}
