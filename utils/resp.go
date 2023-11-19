package utils

import (
	"github.com/gin-gonic/gin"
	"net/http"
)

func FormatErrorResponse(ctx *gin.Context) {
	ctx.JSON(http.StatusBadRequest, gin.H{
		"code": http.StatusBadRequest,
		"msg":  "格式错误",
	})
}
