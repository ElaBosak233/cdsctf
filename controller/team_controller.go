package controller

import "github.com/gin-gonic/gin"

type TeamController interface {
	Create(ctx *gin.Context)
	Update(ctx *gin.Context)
	Delete(ctx *gin.Context)
	Join(ctx *gin.Context)
	Quit(ctx *gin.Context)
	FindById(ctx *gin.Context)
	FindAll(ctx *gin.Context)
}
