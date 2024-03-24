package controller

import (
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"github.com/elabosak233/cloudsdale/internal/service"
	"github.com/elabosak233/cloudsdale/internal/utils/convertor"
	"github.com/gin-gonic/gin"
	"net/http"
)

type IGroupController interface {
	Find(ctx *gin.Context)
	Update(ctx *gin.Context)
}

type GroupController struct {
	groupService service.IGroupService
}

func NewGroupController(appService *service.Service) IGroupController {
	return &GroupController{groupService: appService.GroupService}
}

// Find
// @Summary Find groups
// @Tags Group
// @Accept json
// @Produce json
// @Param 查找请求 query request.GroupFindRequest false "GroupFindRequest"
// @Router /groups/ [get]
func (g *GroupController) Find(ctx *gin.Context) {
	req := request.GroupFindRequest{}
	err := ctx.ShouldBindQuery(&req)
	groups, err := g.groupService.Find(req)
	if err != nil {
		ctx.JSON(http.StatusInternalServerError, gin.H{
			"code": http.StatusInternalServerError,
			"msg":  err.Error(),
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
		"data": groups,
	})
}

// Update
// @Summary Update group
// @Tags Group
// @Accept json
// @Produce json
// @Param id path int true "ID"
// @Param 更新请求 body request.GroupUpdateRequest true "GroupUpdateRequest"
// @Router /groups/{id} [put]
func (g *GroupController) Update(ctx *gin.Context) {
	req := request.GroupUpdateRequest{}
	err := ctx.ShouldBindJSON(&req)
	req.ID = convertor.ToUintD(ctx.Param("id"), 0)
	err = g.groupService.Update(req)
	if err != nil {
		ctx.JSON(http.StatusInternalServerError, gin.H{
			"code": http.StatusInternalServerError,
			"msg":  err.Error(),
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}
