package controller

import (
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"github.com/elabosak233/cloudsdale/internal/service"
	"github.com/elabosak233/cloudsdale/internal/utils/validator"
	"github.com/gin-gonic/gin"
	"net/http"
)

type IConfigController interface {
	Find(ctx *gin.Context)
	Update(ctx *gin.Context)
}

type ConfigController struct {
	configService service.IConfigService
}

func NewConfigController(appService *service.Service) IConfigController {
	return &ConfigController{
		configService: appService.ConfigService,
	}
}

// Find
// @Summary 配置全部查询
// @Description	配置全部查询
// @Tags Config
// @Accept json
// @Produce json
// @Router /configs/ [get]
func (c *ConfigController) Find(ctx *gin.Context) {
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
		"data": c.configService.FindAll(),
	})
}

// Update
// @Summary 更新配置
// @Description	更新配置
// @Tags Config
// @Accept json
// @Produce json
// @Security ApiKeyAuth
// @Param input body request.ConfigUpdateRequest true "body"
// @Router /configs/ [put]
func (c *ConfigController) Update(ctx *gin.Context) {
	configUpdateRequest := request.ConfigUpdateRequest{}
	err := ctx.ShouldBindJSON(&configUpdateRequest)
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusBadRequest,
			"msg":  validator.GetValidMsg(err, &configUpdateRequest),
		})
		return
	}
	if err := c.configService.Update(configUpdateRequest); err != nil {
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
