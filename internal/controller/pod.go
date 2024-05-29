package controller

import (
	"fmt"
	"github.com/elabosak233/cloudsdale/internal/cache"
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"github.com/elabosak233/cloudsdale/internal/service"
	"github.com/elabosak233/cloudsdale/internal/utils"
	"github.com/elabosak233/cloudsdale/internal/utils/convertor"
	"github.com/elabosak233/cloudsdale/internal/utils/validator"
	"github.com/gin-gonic/gin"
	"net/http"
	"time"
)

type IPodController interface {
	Create(ctx *gin.Context)
	Remove(ctx *gin.Context)
	Renew(ctx *gin.Context)
	Find(ctx *gin.Context)
}

type PodController struct {
	podService service.IPodService
}

func NewPodController(appService *service.Service) IPodController {
	return &PodController{
		podService: appService.PodService,
	}
}

// Create
// @Summary 创建实例
// @Description	创建实例
// @Tags Pod
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Param input	body	request.PodCreateRequest	true	"PodCreateRequest"
// @Router /pods/ [post]
func (c *PodController) Create(ctx *gin.Context) {
	podCreateRequest := request.PodCreateRequest{}
	if err := ctx.ShouldBindJSON(&podCreateRequest); err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &podCreateRequest),
		})
		return
	}
	user := ctx.MustGet("user").(*model.User)
	podCreateRequest.UserID = user.ID
	pod, err := c.podService.Create(podCreateRequest)
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	cache.C().DeleteByPrefix("pods")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
		"data": pod,
	})
}

// Remove
// @Summary 停止并删除容器
// @Description	停止并删除容器
// @Tags Pod
// @Produce json
// @Security ApiKeyAuth
// @Param input	body request.PodRemoveRequest true "PodRemoveRequest"
// @Router /pods/{id} [delete]
func (c *PodController) Remove(ctx *gin.Context) {
	instanceRemoveRequest := request.PodRemoveRequest{}
	err := ctx.ShouldBindJSON(&instanceRemoveRequest)
	instanceRemoveRequest.ID = convertor.ToUintD(ctx.Param("id"), 0)
	user := ctx.MustGet("user").(*model.User)
	instanceRemoveRequest.UserID = user.ID
	err = c.podService.Remove(instanceRemoveRequest)
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	cache.C().DeleteByPrefix("pods")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
	})
}

// Renew
// @Summary 容器续期
// @Description	容器续期
// @Tags Pod
// @Produce json
// @Security ApiKeyAuth
// @Param input	body request.PodRenewRequest true "PodRenewRequest"
// @Router /pods/{id} [put]
func (c *PodController) Renew(ctx *gin.Context) {
	instanceRenewRequest := request.PodRenewRequest{}
	err := ctx.ShouldBindJSON(&instanceRenewRequest)
	instanceRenewRequest.ID = convertor.ToUintD(ctx.Param("id"), 0)
	user := ctx.MustGet("user").(*model.User)
	instanceRenewRequest.UserID = user.ID
	removedAt, err := c.podService.Renew(instanceRenewRequest)
	if err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
		return
	}
	cache.C().DeleteByPrefix("pods")
	ctx.JSON(http.StatusOK, gin.H{
		"code":       http.StatusOK,
		"removed_at": removedAt,
	})
}

// Find
// @Summary 实例查询
// @Description	实例查询
// @Tags Pod
// @Produce json
// @Security ApiKeyAuth
// @Param input	query request.PodFindRequest false "PodFindRequest"
// @Router /pods/ [get]
func (c *PodController) Find(ctx *gin.Context) {
	podFindRequest := request.PodFindRequest{}
	if err := ctx.ShouldBindQuery(&podFindRequest); err != nil {
		ctx.JSON(http.StatusBadRequest, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &podFindRequest),
		})
		return
	}
	value, exist := cache.C().Get(fmt.Sprintf("pods:%s", utils.HashStruct(podFindRequest)))
	if !exist {
		pods, _ := c.podService.Find(podFindRequest)
		value = gin.H{
			"code": http.StatusOK,
			"data": pods,
		}
		cache.C().Set(
			fmt.Sprintf("pods:%s", utils.HashStruct(podFindRequest)),
			value,
			5*time.Minute,
		)
	}
	ctx.JSON(http.StatusOK, value)
}
