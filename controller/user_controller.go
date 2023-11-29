package controller

import (
	"github.com/gin-gonic/gin"
)

type UserController interface {
	Login(ctx *gin.Context)
	Logout(ctx *gin.Context)
	Register(ctx *gin.Context)
	Create(ctx *gin.Context)
	Update(ctx *gin.Context)
	Delete(ctx *gin.Context)
	FindById(ctx *gin.Context)
	FindByUsername(ctx *gin.Context)
	FindAll(ctx *gin.Context)
}
