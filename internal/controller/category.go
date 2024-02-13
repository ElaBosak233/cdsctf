package controller

import (
	"github.com/elabosak233/pgshub/internal/model"
	"github.com/elabosak233/pgshub/internal/service"
	"github.com/elabosak233/pgshub/pkg/validator"
	"github.com/gin-gonic/gin"
	"net/http"
)

type ICategoryController interface {
	Create(ctx *gin.Context)
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
// @Router /api/challenges/ [post]
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
