package controllers

import (
	"github.com/elabosak233/pgshub/internal/models/request"
	"github.com/elabosak233/pgshub/internal/services"
	"github.com/elabosak233/pgshub/internal/utils"
	"github.com/gin-gonic/gin"
	"net/http"
)

type TeamController interface {
	Create(ctx *gin.Context)
	Update(ctx *gin.Context)
	Delete(ctx *gin.Context)
	Find(ctx *gin.Context)
	Join(ctx *gin.Context)
	Quit(ctx *gin.Context)
	FindById(ctx *gin.Context)
}

type TeamControllerImpl struct {
	TeamService services.TeamService
}

func NewTeamControllerImpl(appService *services.AppService) TeamController {
	return &TeamControllerImpl{
		TeamService: appService.TeamService,
	}
}

// Create
// @Summary 创建团队
// @Description 创建团队
// @Tags 团队
// @Accept json
// @Produce json
// @Param input body request.TeamCreateRequest true "TeamCreateRequest"
// @Router /api/teams/ [post]
func (c *TeamControllerImpl) Create(ctx *gin.Context) {
	createTeamRequest := request.TeamCreateRequest{}
	err := ctx.ShouldBindJSON(&createTeamRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  utils.GetValidMsg(err, &createTeamRequest),
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
// @Description 更新团队
// @Tags 团队
// @Accept json
// @Produce json
// @Param input body request.TeamUpdateRequest true "TeamUpdateRequest"
// @Router /api/teams/ [put]
func (c *TeamControllerImpl) Update(ctx *gin.Context) {
	updateTeamRequest := request.TeamUpdateRequest{}
	err := ctx.ShouldBindJSON(&updateTeamRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  utils.GetValidMsg(err, &updateTeamRequest),
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
// @Description 删除团队
// @Tags 团队
// @Accept json
// @Produce json
// @Param input body request.TeamDeleteRequest true "TeamDeleteRequest"
// @Router /api/teams/ [delete]
func (c *TeamControllerImpl) Delete(ctx *gin.Context) {
	deleteTeamRequest := request.TeamDeleteRequest{}
	err := ctx.ShouldBindJSON(&deleteTeamRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  utils.GetValidMsg(err, &deleteTeamRequest),
		})
		return
	}
	err = c.TeamService.Delete(deleteTeamRequest.TeamId)
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
// @Description 查找团队
// @Tags 团队
// @Accept json
// @Produce json
// @Param input query request.TeamFindRequest false "TeamFindRequest"
// @Router /api/teams/ [get]
func (c *TeamControllerImpl) Find(ctx *gin.Context) {
	teamData, pageCount, _ := c.TeamService.Find(request.TeamFindRequest{
		TeamName:  ctx.Query("name"),
		CaptainId: ctx.Query("captain_id"),
		Page:      utils.ParseIntParam(ctx.Query("page"), -1),
		Size:      utils.ParseIntParam(ctx.Query("size"), -1),
	})
	ctx.JSON(http.StatusOK, gin.H{
		"code":  http.StatusOK,
		"pages": pageCount,
		"data":  teamData,
	})
}

// Join
// @Summary 加入团队
// @Description 加入团队
// @Tags 团队
// @Accept json
// @Produce json
// @Param input body request.TeamJoinRequest true "TeamJoinRequest"
// @Router /api/teams/members/ [post]
func (c *TeamControllerImpl) Join(ctx *gin.Context) {
	joinTeamRequest := request.TeamJoinRequest{}
	err := ctx.ShouldBindJSON(&joinTeamRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  utils.GetValidMsg(err, &joinTeamRequest),
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
// @Description 退出团队
// @Tags 团队
// @Accept json
// @Produce json
// @Param input body request.TeamQuitRequest true "TeamQuitRequest"
// @Router /api/teams/members/ [delete]
func (c *TeamControllerImpl) Quit(ctx *gin.Context) {
	quitTeamRequest := request.TeamQuitRequest{}
	err := ctx.ShouldBindJSON(&quitTeamRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  utils.GetValidMsg(err, &quitTeamRequest),
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
// @Description 查找团队
// @Tags 团队
// @Accept json
// @Produce json
// @Param id path string true "id"
// @Router /api/teams/id/{id} [get]
func (c *TeamControllerImpl) FindById(ctx *gin.Context) {
	id := ctx.Param("id")
	res, err := c.TeamService.FindById(id)
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
