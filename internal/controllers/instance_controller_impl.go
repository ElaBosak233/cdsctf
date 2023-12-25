package controllers

import (
	"github.com/elabosak233/pgshub/internal/models/request"
	"github.com/elabosak233/pgshub/internal/services"
	"github.com/elabosak233/pgshub/internal/utils"
	"github.com/gin-gonic/gin"
	"net/http"
)

type InstanceControllerImpl struct {
	instanceService services.InstanceService
}

func NewInstanceControllerImpl(appService *services.AppService) InstanceController {
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
// @Router /api/instances/ [post]
func (c *InstanceControllerImpl) Create(ctx *gin.Context) {
	instanceCreateRequest := request.InstanceCreateRequest{}
	err := ctx.ShouldBindJSON(&instanceCreateRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  utils.GetValidMsg(err, &instanceCreateRequest),
		})
		return
	}
	id, entry := c.instanceService.Create(instanceCreateRequest.ChallengeId)
	ctx.JSON(http.StatusOK, gin.H{
		"code":  http.StatusOK,
		"id":    id,
		"entry": entry,
	})
}

// Remove
// @Summary 停止并删除容器
// @Description 停止并删除容器
// @Tags 实例
// @Produce json
// @Param input body request.InstanceRemoveRequest true "InstanceRemoveRequest"
// @Router /api/instances/ [delete]
func (c *InstanceControllerImpl) Remove(ctx *gin.Context) {
	instanceRemoveRequest := request.InstanceRemoveRequest{}
	err := ctx.ShouldBindJSON(&instanceRemoveRequest)
	err = c.instanceService.Remove(instanceRemoveRequest.InstanceId)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
	} else {
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
// @Param input body request.InstanceRenewRequest true "InstanceRenewRequest"
// @Router /api/instances/ [put]
func (c *InstanceControllerImpl) Renew(ctx *gin.Context) {
	instanceRenewRequest := request.InstanceRenewRequest{}
	err := ctx.ShouldBindJSON(&instanceRenewRequest)
	err = c.instanceService.Renew(instanceRenewRequest.InstanceId)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  err.Error(),
		})
	} else {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusOK,
		})
	}
}

// FindById
// @Summary 实例查询
// @Description 实例查询
// @Tags 实例
// @Produce json
// @Param id path string true "id"
// @Router /api/instances/{id} [get]
func (c *InstanceControllerImpl) FindById(ctx *gin.Context) {
	id := ctx.Param("id")
	rep, err := c.instanceService.FindById(id)
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

// FindAll
// @Summary 实例全部查询
// @Description 实例全部查询
// @Tags 实例
// @Produce json
// @Router /api/instances/ [get]
func (c *InstanceControllerImpl) FindAll(ctx *gin.Context) {
	rep, _ := c.instanceService.FindAll()
	res := make([]map[string]any, len(rep))
	for i, v := range rep {
		item := map[string]any{
			"id":           v.InstanceId,
			"challenge_id": v.ChallengeId,
			"status":       v.Status,
			"entry":        v.Entry,
			"remove_at":    v.RemoveAt,
		}
		res[i] = item
	}
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
		"data": res,
	})
}
