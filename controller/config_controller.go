package controller

import "github.com/gin-gonic/gin"

type ConfigController interface {
	Get(ctx *gin.Context)
}
