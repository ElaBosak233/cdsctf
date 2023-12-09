package controller

import (
	"github.com/elabosak233/pgshub/model/request"
	"github.com/elabosak233/pgshub/service"
	"github.com/elabosak233/pgshub/utils"
	"github.com/gin-gonic/gin"
	"net/http"
)

type InstanceControllerImpl struct {
	instanceService service.InstanceService
}

func NewInstanceControllerImpl(appService *service.AppService) InstanceController {
	return &InstanceControllerImpl{
		instanceService: appService.InstanceService,
	}
}

// Create
// @Summary 创建实例
// @Description 创建实例
// @Tags 实例
// @Accept json
// @Produce json
// @Param input body request.InstanceCreateRequest true "InstanceCreateRequest"
// @Router /api/instance/create [post]
func (c *InstanceControllerImpl) Create(ctx *gin.Context) {
	instanceCreateRequest := request.InstanceCreateRequest{}
	err := ctx.ShouldBindJSON(&instanceCreateRequest)
	if err != nil {
		ctx.Header("Content-Type", "application/json")
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  utils.GetValidMsg(err, &instanceCreateRequest),
		})
		return
	}
	id, entry := c.instanceService.Create(instanceCreateRequest.ChallengeId)
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, gin.H{
		"code":  http.StatusOK,
		"id":    id,
		"entry": entry,
	})
}

// Status
// @Summary 获取实例状态
// @Description 获取实例状态
// @Tags 实例
// @Produce json
// @Param id query string true "InstanceId"
// @Router /api/instance/status [get]
func (c *InstanceControllerImpl) Status(ctx *gin.Context) {
	id := ctx.Query("id")
	status, entry, err := c.instanceService.Status(id)
	if err != nil {
		ctx.Header("Content-Type", "application/json")
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
	} else {
		ctx.Header("Content-Type", "application/json")
		ctx.JSON(http.StatusOK, gin.H{
			"code":   http.StatusOK,
			"status": status,
			"entry":  entry,
		})
	}
}

// Remove
// @Summary 停止并删除容器
// @Description 停止并删除容器
// @Tags 实例
// @Produce json
// @Param id query string true "InstanceId"
// @Router /api/instance/remove [get]
func (c *InstanceControllerImpl) Remove(ctx *gin.Context) {
	id := ctx.Query("id")
	err := c.instanceService.Remove(id)
	if err != nil {
		ctx.Header("Content-Type", "application/json")
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
	} else {
		ctx.Header("Content-Type", "application/json")
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusOK,
		})
	}
}

// Renew
// @Summary 容器续期
// @Description 容器续期
// @Tags 实例
// @Produce json
// @Param id query string true "InstanceId"
// @Router /api/instance/renew [get]
func (c *InstanceControllerImpl) Renew(ctx *gin.Context) {
	id := ctx.Query("id")
	err := c.instanceService.Renew(id)
	if err != nil {
		ctx.Header("Content-Type", "application/json")
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
	} else {
		ctx.Header("Content-Type", "application/json")
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusOK,
		})
	}
}
