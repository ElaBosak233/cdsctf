package controllers

import "github.com/gin-gonic/gin"

type GameController interface {
	Create(ctx *gin.Context)
	Delete(ctx *gin.Context)
	Update(ctx *gin.Context)
	ApplyByTeamId(ctx *gin.Context)
}
