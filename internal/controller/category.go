package controller

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/dto/request"
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
}

type CategoryController struct {
	CategoryService service.ICategoryService
}

func NewCategoryController(appService *service.Service) ICategoryController {
	return &CategoryController{
		CategoryService: appService.CategoryService,
	}
}

// Create
// @Summary create new category
// @Description
// @Tags Category
// @Accept json
// @Produce json
// @Param Authorization header string true "Authorization"
// @Param req body request.CategoryCreateRequest true "CategoryCreateRequest"
// @Router /api/categories/ [post]
func (c *CategoryController) Create(ctx *gin.Context) {
	req := model.Category{}
	if err := ctx.ShouldBindJSON(&req); err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &req),
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

// Update
// @Summary update category
// @Description
// @Tags Category
// @Accept json
// @Produce json
// @Param Authorization header string true "Authorization"
// @Param req body request.CategoryUpdateRequest true "CategoryUpdateRequest"
// @Router /api/categories/ [put]
func (c *CategoryController) Update(ctx *gin.Context) {
	req := model.Category{}
	if err := ctx.ShouldBindJSON(&req); err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &req),
		})
		return
	}
	err := c.CategoryService.Update(req)
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

// Find
// @Summary get category
// @Description
// @Tags Category
// @Accept json
// @Produce json
// @Param Authorization header string true "Authorization"
// @Param req query request.CategoryFindRequest true "CategoryFindRequest"
// @Router /api/categories/ [get]
func (c *CategoryController) Find(ctx *gin.Context) {
	req := request.CategoryFindRequest{
		ID:   convertor.ToUintD(ctx.Query("id"), 0),
		Name: ctx.Query("name"),
	}
	res, err := c.CategoryService.Find(req)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
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
