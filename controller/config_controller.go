package controller

import "github.com/gin-gonic/gin"

type ConfigController interface {
	FindAll(ctx *gin.Context)
}
