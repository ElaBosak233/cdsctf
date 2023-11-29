package m2m

import "github.com/gin-gonic/gin"

type UserGroupController interface {
	Create(ctx *gin.Context)
	Delete(ctx *gin.Context)
}
