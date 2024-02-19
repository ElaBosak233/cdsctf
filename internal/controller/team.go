package controller

import (
	"github.com/elabosak233/cloudsdale/internal/model/dto/request"
	"github.com/elabosak233/cloudsdale/internal/service"
	"github.com/elabosak233/cloudsdale/internal/utils/convertor"
	"github.com/elabosak233/cloudsdale/internal/utils/validator"
	"github.com/gin-gonic/gin"
	"net/http"
)

type ITeamController interface {
	Create(ctx *gin.Context)
	Update(ctx *gin.Context)
	Delete(ctx *gin.Context)
	Find(ctx *gin.Context)
	BatchFind(ctx *gin.Context)
	Join(ctx *gin.Context)
	Quit(ctx *gin.Context)
	FindById(ctx *gin.Context)
}

type TeamController struct {
	TeamService service.ITeamService
}

func NewTeamController(appService *service.Service) ITeamController {
	return &TeamController{
		TeamService: appService.TeamService,
	}
}

// Create
// @Summary 创建团队
// @Description	创建团队
// @Tags Team
// @Accept json
// @Produce json
// @Param input	body	request.TeamCreateRequest	true	"TeamCreateRequest"
// @Router /teams/ [post]
func (c *TeamController) Create(ctx *gin.Context) {
	createTeamRequest := request.TeamCreateRequest{}
	err := ctx.ShouldBindJSON(&createTeamRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &createTeamRequest),
		})
		return
	}
	err = c.TeamService.Create(createTeamRequest)
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
// @Summary 更新团队
// @Description	更新团队
// @Tags Team
// @Accept json
// @Produce json
// @Param input	body request.TeamUpdateRequest true "TeamUpdateRequest"
// @Router /teams/ [put]
func (c *TeamController) Update(ctx *gin.Context) {
	updateTeamRequest := request.TeamUpdateRequest{}
	err := ctx.ShouldBindJSON(&updateTeamRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &updateTeamRequest),
		})
		return
	}
	err = c.TeamService.Update(updateTeamRequest)
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

// Delete
// @Summary 删除团队
// @Description	删除团队
// @Tags Team
// @Accept json
// @Produce json
// @Param input	body	request.TeamDeleteRequest	true	"TeamDeleteRequest"
// @Router /teams/ [delete]
func (c *TeamController) Delete(ctx *gin.Context) {
	deleteTeamRequest := request.TeamDeleteRequest{}
	err := ctx.ShouldBindJSON(&deleteTeamRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &deleteTeamRequest),
		})
		return
	}
	err = c.TeamService.Delete(deleteTeamRequest.ID)
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
// @Summary 查找团队
// @Description	查找团队
// @Tags Team
// @Accept json
// @Produce json
// @Param input	query request.TeamFindRequest false	"TeamFindRequest"
// @Router /teams/ [get]
func (c *TeamController) Find(ctx *gin.Context) {
	teamData, pageCount, total, _ := c.TeamService.Find(request.TeamFindRequest{
		ID:        convertor.ToUintD(ctx.Query("id"), 0),
		TeamName:  ctx.Query("name"),
		CaptainID: convertor.ToUintD(ctx.Query("captain_id"), 0),
		Page:      convertor.ToIntD(ctx.Query("page"), 0),
		Size:      convertor.ToIntD(ctx.Query("size"), 0),
	})
	ctx.JSON(http.StatusOK, gin.H{
		"code":  http.StatusOK,
		"pages": pageCount,
		"total": total,
		"data":  teamData,
	})
}

// BatchFind
// @Summary 批量查找团队
// @Description	批量查找团队
// @Tags Team
// @Accept json
// @Produce json
// @Param input	query request.TeamBatchFindRequest false "TeamBatchFindRequest"
// @Router /teams/batch/ [get]
func (c *TeamController) BatchFind(ctx *gin.Context) {
	teams, _ := c.TeamService.BatchFind(request.TeamBatchFindRequest{
		ID: convertor.ToInt64SliceD(ctx.QueryArray("id"), []int64{}),
	})
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
		"data": teams,
	})
}

// Join
// @Summary 加入团队
// @Description	加入团队
// @Tags Team
// @Accept json
// @Produce json
// @Param input	body	request.TeamJoinRequest	true	"TeamJoinRequest"
// @Router /teams/members/ [post]
func (c *TeamController) Join(ctx *gin.Context) {
	joinTeamRequest := request.TeamJoinRequest{}
	err := ctx.ShouldBindJSON(&joinTeamRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &joinTeamRequest),
		})
		return
	}
	err = c.TeamService.Join(joinTeamRequest)
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

// Quit
// @Summary 退出团队
// @Description	退出团队
// @Tags Team
// @Accept json
// @Produce json
// @Param input	body request.TeamQuitRequest true "TeamQuitRequest"
// @Router /teams/members/ [delete]
func (c *TeamController) Quit(ctx *gin.Context) {
	quitTeamRequest := request.TeamQuitRequest{}
	err := ctx.ShouldBindJSON(&quitTeamRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &quitTeamRequest),
		})
		return
	}
	err = c.TeamService.Quit(quitTeamRequest)
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

// FindById
// @Summary 查找团队
// @Description	查找团队
// @Tags Team
// @Accept json
// @Produce json
// @Param id path string true "id"
// @Router /teams/id/{id} [get]
func (c *TeamController) FindById(ctx *gin.Context) {
	id := ctx.Param("id")
	res, err := c.TeamService.FindById(convertor.ToUintD(id, 0))
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
