package controllers

import (
	"github.com/elabosak233/pgshub/models/entity"
	"github.com/elabosak233/pgshub/services"
	"github.com/gin-gonic/gin"
	"net/http"
)

type CategoryController interface {
	Create(ctx *gin.Context)
}

type CategoryControllerImpl struct {
	CategoryService services.CategoryService
}

func NewCategoryController(appService *services.Services) CategoryController {
	return &CategoryControllerImpl{
		CategoryService: appService.CategoryService,
	}
}

func (c *CategoryControllerImpl) Create(ctx *gin.Context) {
	req := entity.Category{}
	if err := ctx.ShouldBindJSON(&req); err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	err := c.CategoryService.Create(req)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}
