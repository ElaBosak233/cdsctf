package implements

import (
	"github.com/elabosak233/pgshub/internal/controllers"
	"github.com/elabosak233/pgshub/internal/services"
	"github.com/gin-gonic/gin"
	"net/http"
)

type ConfigControllerImpl struct {
	ConfigService services.ConfigService
}

func NewConfigControllerImpl(appService *services.AppService) controllers.ConfigController {
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
		"code":   http.StatusOK,
		"config": c.ConfigService.FindAll(),
	})
}

// Update
// @Summary 更新配置
// @Description 更新配置
// @Tags 配置
// @Accept json
// @Produce json
// @Router /api/configs/ [put]
func (c *ConfigControllerImpl) Update(ctx *gin.Context) {

}
