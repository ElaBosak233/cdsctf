package controller

import (
	"github.com/elabosak233/pgshub/service"
	"github.com/gin-gonic/gin"
	"net/http"
)

type ConfigControllerImpl struct {
	configService service.ConfigService
}

func NewConfigControllerImpl(appService *service.AppService) ConfigController {
	return &ConfigControllerImpl{
		appService.ConfigService,
	}
}

// FindAll
// @Summary 配置全部查询
// @Description 配置全部查询
// @Tags 配置
// @Accept json
// @Produce json
// @Router /api/configs [get]
func (c *ConfigControllerImpl) FindAll(ctx *gin.Context) {
	ctx.JSON(http.StatusOK, gin.H{
		"code":   http.StatusOK,
		"config": c.configService.FindAll(),
	})
}

// Update
// @Summary 更新配置
// @Description 更新配置
// @Tags 配置
// @Accept json
// @Produce json
// @Router /api/configs [put]
func (c *ConfigControllerImpl) Update(ctx *gin.Context) {

}
