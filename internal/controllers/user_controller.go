package controllers

import (
	"github.com/gin-gonic/gin"
)

type UserController interface {
	Login(ctx *gin.Context)
	VerifyToken(ctx *gin.Context)
	Logout(ctx *gin.Context)
	Register(ctx *gin.Context)
	Create(ctx *gin.Context)
	Update(ctx *gin.Context)
	Delete(ctx *gin.Context)
	Find(ctx *gin.Context)
}
