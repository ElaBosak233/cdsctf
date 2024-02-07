package controllers

import (
	"github.com/elabosak233/pgshub/models/request"
	"github.com/elabosak233/pgshub/services"
	"github.com/elabosak233/pgshub/utils/convertor"
	"github.com/elabosak233/pgshub/utils/validator"
	"github.com/gin-gonic/gin"
	"net/http"
)

type PodController interface {
	Create(ctx *gin.Context)
	Remove(ctx *gin.Context)
	Renew(ctx *gin.Context)
	Find(ctx *gin.Context)
	FindById(ctx *gin.Context)
}

type PodControllerImpl struct {
	PodService services.PodService
}

func NewInstanceControllerImpl(appService *services.Services) PodController {
	return &PodControllerImpl{
		PodService: appService.PodService,
	}
}

// Create
// @Summary 创建实例
// @Description 创建实例
// @Tags 实例
// @Accept json
// @Produce json
// @Param Authorization header string true "Authorization"
// @Param input body request.InstanceCreateRequest true "InstanceCreateRequest"
// @Router /api/pods/ [post]
func (c *PodControllerImpl) Create(ctx *gin.Context) {
	instanceCreateRequest := request.InstanceCreateRequest{}
	err := ctx.ShouldBindJSON(&instanceCreateRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &instanceCreateRequest),
		})
		return
	}
	instanceCreateRequest.UserId = ctx.GetInt64("UserID")
	res, err := c.PodService.Create(instanceCreateRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code":       http.StatusOK,
		"id":         res.PodID,
		"containers": res.Containers,
		"removed_at": res.RemovedAt,
	})
}

// Remove
// @Summary 停止并删除容器
// @Description 停止并删除容器
// @Tags 实例
// @Produce json
// @Param Authorization header string true "Authorization"
// @Param input body request.PodRemoveRequest true "PodRemoveRequest"
// @Router /api/pods/ [delete]
func (c *PodControllerImpl) Remove(ctx *gin.Context) {
	instanceRemoveRequest := request.PodRemoveRequest{}
	err := ctx.ShouldBindJSON(&instanceRemoveRequest)
	instanceRemoveRequest.UserId = ctx.GetInt64("UserID")
	err = c.PodService.Remove(instanceRemoveRequest)
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

// Renew
// @Summary 容器续期
// @Description 容器续期
// @Tags 实例
// @Produce json
// @Param Authorization header string true "Authorization"
// @Param input body request.InstanceRenewRequest true "InstanceRenewRequest"
// @Router /api/pods/ [put]
func (c *PodControllerImpl) Renew(ctx *gin.Context) {
	instanceRenewRequest := request.InstanceRenewRequest{}
	err := ctx.ShouldBindJSON(&instanceRenewRequest)
	instanceRenewRequest.UserId = ctx.GetInt64("UserID")
	removedAt, err := c.PodService.Renew(instanceRenewRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code":       http.StatusOK,
		"removed_at": removedAt,
	})
}

// FindById
// @Summary 实例查询
// @Description 实例查询
// @Tags 实例
// @Produce json
// @Param id path string true "id"
// @Router /api/pods/{id} [get]
func (c *PodControllerImpl) FindById(ctx *gin.Context) {
	id := ctx.Param("id")
	rep, err := c.PodService.FindById(int64(convertor.ToIntD(id, 0)))
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
		"data": rep,
	})
}

// Find
// @Summary 实例查询
// @Description 实例查询
// @Tags 实例
// @Produce json
// @Param Authorization header string true "Authorization"
// @Param input query request.PodFindRequest false "PodFindRequest"
// @Router /api/pods/ [get]
func (c *PodControllerImpl) Find(ctx *gin.Context) {
	podFindRequest := request.PodFindRequest{
		UserId:      ctx.GetInt64("UserID"),
		ChallengeId: int64(convertor.ToIntD(ctx.Query("challenge_id"), 0)),
		TeamId:      int64(convertor.ToIntD(ctx.Query("team_id"), 0)),
		GameId:      int64(convertor.ToIntD(ctx.Query("game_id"), 0)),
		IsAvailable: convertor.ToBoolP(ctx.Query("is_available")),
		Page:        convertor.ToIntD(ctx.Query("page"), 0),
		Size:        convertor.ToIntD(ctx.Query("size"), 0),
	}
	pods, _ := c.PodService.Find(podFindRequest)
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
		"data": pods,
	})
}
