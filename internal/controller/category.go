package controller

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"github.com/elabosak233/cloudsdale/internal/service"
	"github.com/elabosak233/cloudsdale/internal/utils/convertor"
	"github.com/elabosak233/cloudsdale/internal/utils/validator"
	"github.com/gin-gonic/gin"
	"net/http"
)

type ICategoryController interface {
	Create(ctx *gin.Context)
	Update(ctx *gin.Context)
	Find(ctx *gin.Context)
	Delete(ctx *gin.Context)
}

type CategoryController struct {
	categoryService service.ICategoryService
}

func NewCategoryController(appService *service.Service) ICategoryController {
	return &CategoryController{
		categoryService: appService.CategoryService,
	}
}

// Create
// @Summary create new category
// @Description
// @Tags Category
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Param req body request.CategoryCreateRequest true "CategoryCreateRequest"
// @Router /categories/ [post]
func (c *CategoryController) Create(ctx *gin.Context) {
	req := model.Category{}
	if err := ctx.ShouldBindJSON(&req); err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &req),
		})
		return
	}
	err := c.categoryService.Create(req)
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

// Update
// @Summary update category
// @Description
// @Tags Category
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Param req body request.CategoryUpdateRequest true "CategoryUpdateRequest"
// @Router /categories/{id} [put]
func (c *CategoryController) Update(ctx *gin.Context) {
	req := model.Category{}
	if err := ctx.ShouldBindJSON(&req); err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &req),
		})
		return
	}
	req.ID = convertor.ToUintD(ctx.Param("id"), 0)
	err := c.categoryService.Update(req)
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

// Find
// @Summary get category
// @Description
// @Tags Category
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Param req query request.CategoryFindRequest	true "CategoryFindRequest"
// @Router /categories/ [get]
func (c *CategoryController) Find(ctx *gin.Context) {
	req := request.CategoryFindRequest{}
	err := ctx.ShouldBindQuery(&req)
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &req),
		})
		return
	}
	res, err := c.categoryService.Find(req)
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
		"data": res,
	})
}

// Delete
// @Summary delete category
// @Description
// @Tags Category
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Param req body request.CategoryDeleteRequest true "CategoryDeleteRequest"
// @Router /categories/{id} [delete]
func (c *CategoryController) Delete(ctx *gin.Context) {
	req := request.CategoryDeleteRequest{}
	req.ID = convertor.ToUintD(ctx.Param("id"), 0)
	err := c.categoryService.Delete(req)
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}
