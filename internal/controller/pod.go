package controller

import (
	"github.com/elabosak233/pgshub/internal/model/dto/request"
	"github.com/elabosak233/pgshub/internal/service"
	"github.com/elabosak233/pgshub/pkg/convertor"
	"github.com/elabosak233/pgshub/pkg/validator"
	"github.com/gin-gonic/gin"
	"net/http"
)

type IPodController interface {
	Create(ctx *gin.Context)
	Remove(ctx *gin.Context)
	Renew(ctx *gin.Context)
	Find(ctx *gin.Context)
	FindById(ctx *gin.Context)
}

type PodController struct {
	PodService service.IPodService
}

func NewInstanceController(appService *service.Service) IPodController {
	return &PodController{
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
// @Param input body request.PodCreateRequest true "PodCreateRequest"
// @Router /api/pods/ [post]
func (c *PodController) Create(ctx *gin.Context) {
	instanceCreateRequest := request.PodCreateRequest{}
	err := ctx.ShouldBindJSON(&instanceCreateRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &instanceCreateRequest),
		})
		return
	}
	instanceCreateRequest.UserID = ctx.GetInt64("ID")
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
		"id":         res.ID,
		"instances":  res.Instances,
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
func (c *PodController) Remove(ctx *gin.Context) {
	instanceRemoveRequest := request.PodRemoveRequest{}
	err := ctx.ShouldBindJSON(&instanceRemoveRequest)
	instanceRemoveRequest.UserID = ctx.GetInt64("ID")
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
// @Param input body request.PodRenewRequest true "PodRenewRequest"
// @Router /api/pods/ [put]
func (c *PodController) Renew(ctx *gin.Context) {
	instanceRenewRequest := request.PodRenewRequest{}
	err := ctx.ShouldBindJSON(&instanceRenewRequest)
	instanceRenewRequest.UserID = ctx.GetInt64("ID")
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
func (c *PodController) FindById(ctx *gin.Context) {
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
func (c *PodController) Find(ctx *gin.Context) {
	podFindRequest := request.PodFindRequest{
		UserID:      ctx.GetInt64("ID"),
		IDs:         convertor.ToInt64SliceD(ctx.QueryArray("id"), []int64{}),
		ChallengeID: int64(convertor.ToIntD(ctx.Query("challenge_id"), 0)),
		TeamID:      int64(convertor.ToIntD(ctx.Query("team_id"), 0)),
		GameID:      int64(convertor.ToIntD(ctx.Query("game_id"), 0)),
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
