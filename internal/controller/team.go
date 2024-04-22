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

type ITeamController interface {
	Create(ctx *gin.Context)
	Update(ctx *gin.Context)
	Delete(ctx *gin.Context)
	Find(ctx *gin.Context)
	CreateUser(ctx *gin.Context)
	DeleteUser(ctx *gin.Context)
	FindById(ctx *gin.Context)
	GetInviteToken(ctx *gin.Context)
	UpdateInviteToken(ctx *gin.Context)
	Join(ctx *gin.Context)
	Leave(ctx *gin.Context)
	SaveAvatar(ctx *gin.Context)
	DeleteAvatar(ctx *gin.Context)
}

type TeamController struct {
	teamService     service.ITeamService
	userTeamService service.IUserTeamService
	mediaService    service.IMediaService
}

func NewTeamController(appService *service.Service) ITeamController {
	return &TeamController{
		teamService:     appService.TeamService,
		userTeamService: appService.UserTeamService,
		mediaService:    appService.MediaService,
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
	if err := ctx.ShouldBindJSON(&updateTeamRequest); err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &updateTeamRequest),
		})
		return
	}
	updateTeamRequest.ID = convertor.ToUintD(ctx.Param("id"), 0)
	err := c.teamService.Update(updateTeamRequest)
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
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &teamFindRequest),
		})
		return
	}
	teams, pages, total, _ := c.teamService.Find(teamFindRequest)
	ctx.JSON(http.StatusOK, gin.H{
		"code":  http.StatusOK,
		"pages": pages,
		"total": total,
		"data":  teams,
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
	team, err := c.teamService.FindById(convertor.ToUintD(id, 0))
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
		"data": team,
	})
}

// CreateUser
// @Summary 加入团队
// @Description	加入团队
// @Tags Team
// @Accept json
// @Produce json
// @Param input	body request.TeamUserCreateRequest true "TeamUserCreateRequest"
// @Router /teams/{id}/users/ [post]
func (c *TeamController) CreateUser(ctx *gin.Context) {
	teamUserCreateRequest := request.TeamUserCreateRequest{
		TeamID: convertor.ToUintD(ctx.Param("id"), 0),
	}
	err := ctx.ShouldBindJSON(&teamUserCreateRequest)
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &teamUserCreateRequest),
		})
		return
	}
	err = c.userTeamService.Create(teamUserCreateRequest)
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

// DeleteUser
// @Summary 踢出团队
// @Description	踢出团队
// @Tags Team
// @Accept json
// @Produce json
// @Param input	body request.TeamUserDeleteRequest true "TeamUserDeleteRequest"
// @Router /teams/{id}/users/{user_id} [delete]
func (c *TeamController) DeleteUser(ctx *gin.Context) {
	teamUserDeleteRequest := request.TeamUserDeleteRequest{
		TeamID: convertor.ToUintD(ctx.Param("id"), 0),
		UserID: convertor.ToUintD(ctx.Param("user_id"), 0),
	}
	err := c.userTeamService.Delete(teamUserDeleteRequest)
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

// GetInviteToken
// @Summary 获取邀请码
// @Description	获取邀请码
// @Tags Team
// @Accept json
// @Produce json
// @Param id path string true "id"
// @Router /teams/{id}/invite [get]
func (c *TeamController) GetInviteToken(ctx *gin.Context) {
	id := ctx.Param("id")
	token, err := c.teamService.GetInviteToken(request.TeamGetInviteTokenRequest{
		ID: convertor.ToUintD(id, 0),
	})
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code":         http.StatusOK,
		"invite_token": token,
	})
}

// UpdateInviteToken
// @Summary 更新邀请码
// @Description	更新邀请码
// @Tags Team
// @Accept json
// @Produce json
// @Param id path string true "id"
// @Router /teams/{id}/invite [put]
func (c *TeamController) UpdateInviteToken(ctx *gin.Context) {
	id := ctx.Param("id")
	teamUpdateInviteTokenRequest := request.TeamUpdateInviteTokenRequest{}
	teamUpdateInviteTokenRequest.ID = convertor.ToUintD(id, 0)
	token, err := c.teamService.UpdateInviteToken(teamUpdateInviteTokenRequest)
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code":         http.StatusOK,
		"invite_token": token,
	})
}

// Join
// @Summary 加入团队
// @Description	加入团队
// @Tags Team
// @Accept json
// @Produce json
// @Param id path string true "id"
// @Router /teams/{id}/join [post]
func (c *TeamController) Join(ctx *gin.Context) {
	id := ctx.Param("id")
	user := ctx.MustGet("user").(*model.User)
	err := c.userTeamService.Create(request.TeamUserCreateRequest{
		TeamID: convertor.ToUintD(id, 0),
		UserID: user.ID,
	})
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

// Leave
// @Summary 离开团队
// @Description	离开团队
// @Tags Team
// @Accept json
// @Produce json
// @Param id path string true "id"
// @Router /teams/{id}/leave [delete]
func (c *TeamController) Leave(ctx *gin.Context) {
	id := ctx.Param("id")
	user := ctx.MustGet("user").(*model.User)
	err := c.userTeamService.Delete(request.TeamUserDeleteRequest{
		TeamID: convertor.ToUintD(id, 0),
		UserID: user.ID,
	})
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

// SaveAvatar
// @Summary 保存头像
// @Description
// @Tags Challenge
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Param file formData file true "avatar"
// @Router /teams/{id}/avatar [post]
func (c *TeamController) SaveAvatar(ctx *gin.Context) {
	id := convertor.ToUintD(ctx.Param("id"), 0)
	fileHeader, err := ctx.FormFile("file")
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	err = c.mediaService.SaveTeamAvatar(id, fileHeader)
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

// DeleteAvatar
// @Summary 删除头像
// @Description
// @Tags Challenge
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Router /teams/{id}/avatar [delete]
func (c *TeamController) DeleteAvatar(ctx *gin.Context) {
	id := convertor.ToUintD(ctx.Param("id"), 0)
	err := c.mediaService.DeleteTeamAvatar(id)
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
