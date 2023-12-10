package controller

import (
	"github.com/elabosak233/pgshub/service"
	"github.com/gin-gonic/gin"
	"github.com/spf13/viper"
	"net/http"
)

type ConfigControllerImpl struct {
}

func NewConfigControllerImpl(appService *service.AppService) ConfigController {
	return &ConfigControllerImpl{}
}

func (c ConfigControllerImpl) Get(ctx *gin.Context) {
	ctx.Header("Content-Type", "application/json")
	ctx.JSON(http.StatusOK, gin.H{
		"code": http.StatusOK,
		"config": map[string]interface{}{
			"title": viper.GetString("Title"),
		},
	})
}
