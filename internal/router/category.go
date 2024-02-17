package router

import (
	"github.com/elabosak233/cloudsdale/internal/controller"
	"github.com/elabosak233/cloudsdale/internal/middleware"
	"github.com/gin-gonic/gin"
)

func NewCategoryRouter(categoryRouter *gin.RouterGroup, categoryController controller.ICategoryController, authMiddleware middleware.IAuthMiddleware) {
	categoryRouter.POST("/", authMiddleware.AuthInRole(2), categoryController.Create)
	categoryRouter.PUT("/", authMiddleware.AuthInRole(2), categoryController.Update)
	categoryRouter.GET("/", authMiddleware.Auth(), categoryController.Find)
}
