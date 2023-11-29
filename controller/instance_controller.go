package controller

import "github.com/gin-gonic/gin"

type InstanceController interface {
	Create(ctx *gin.Context)
	Status(ctx *gin.Context)
	Remove(ctx *gin.Context)
	Renew(ctx *gin.Context)
}
