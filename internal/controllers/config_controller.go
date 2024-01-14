package controllers

import (
	"github.com/elabosak233/pgshub/internal/models/request"
	"github.com/elabosak233/pgshub/internal/services"
	"github.com/elabosak233/pgshub/internal/utils"
	"github.com/gin-gonic/gin"
	"net/http"
)

type ConfigController interface {
	Find(ctx *gin.Context)
	Update(ctx *gin.Context)
}

type ConfigControllerImpl struct {
	ConfigService services.ConfigService
}

func NewConfigControllerImpl(appService *services.AppService) ConfigController {
	return &ConfigControllerImpl{
		appService.ConfigService,
	}
}

// Find
// @Summary 配置全部查询
// @Description 配置全部查询
// @Tags 配置
// @Accept json
// @Produce json
// @Router /api/configs/ [get]
func (c *ConfigControllerImpl) Find(ctx *gin.Context) {
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
		"data": c.ConfigService.FindAll(),
	})
}

// Update
// @Summary 更新配置
// @Description 更新配置
// @Tags 配置
// @Accept json
// @Produce json
// @Param PgsToken header string true "PgsToken"
// @Param input body request.ConfigUpdateRequest true "body"
// @Router /api/configs/ [put]
func (c *ConfigControllerImpl) Update(ctx *gin.Context) {
	configUpdateRequest := request.ConfigUpdateRequest{}
	err := ctx.ShouldBindJSON(&configUpdateRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  utils.GetValidMsg(err, &configUpdateRequest),
		})
		return
	}
	if err := c.ConfigService.Update(configUpdateRequest); err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusInternalServerError,
			"msg":  err.Error(),
		})
	} else {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusOK,
			"msg":  "更新成功",
		})
	}
}
