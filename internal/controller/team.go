package controller

import (
	"github.com/elabosak233/cloudsdale/internal/model/request"
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
	Join(ctx *gin.Context)
	Quit(ctx *gin.Context)
	FindById(ctx *gin.Context)
}

type TeamController struct {
	teamService service.ITeamService
}

func NewTeamController(appService *service.Service) ITeamController {
	return &TeamController{
		teamService: appService.TeamService,
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
	err = c.teamService.Create(createTeamRequest)
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
// @Router /teams/{id} [put]
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
	updateTeamRequest.ID = convertor.ToUintD(ctx.Param("id"), 0)
	err = c.teamService.Update(updateTeamRequest)
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
// @Param input	body request.TeamDeleteRequest true "TeamDeleteRequest"
// @Router /teams/{id} [delete]
func (c *TeamController) Delete(ctx *gin.Context) {
	deleteTeamRequest := request.TeamDeleteRequest{}
	deleteTeamRequest.ID = convertor.ToUintD(ctx.Param("id"), 0)
	err := c.teamService.Delete(deleteTeamRequest.ID)
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
	teamFindRequest := request.TeamFindRequest{}
	err := ctx.ShouldBindQuery(&teamFindRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &teamFindRequest),
		})
		return
	}
	teamData, pageCount, total, _ := c.teamService.Find(teamFindRequest)
	ctx.JSON(http.StatusOK, gin.H{
		"code":  http.StatusOK,
		"pages": pageCount,
		"total": total,
		"data":  teamData,
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
	err = c.teamService.Join(joinTeamRequest)
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
	err = c.teamService.Quit(quitTeamRequest)
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
// @Router /teams/{id} [get]
func (c *TeamController) FindById(ctx *gin.Context) {
	id := ctx.Param("id")
	res, err := c.teamService.FindById(convertor.ToUintD(id, 0))
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
