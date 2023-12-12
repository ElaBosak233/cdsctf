package controller

import "github.com/gin-gonic/gin"

type AssetController interface {
	FindUserAvatarByUserId(ctx *gin.Context)
	SetUserAvatarByUserId(ctx *gin.Context)
	FindTeamAvatarByTeamId(ctx *gin.Context)
	SetTeamAvatarByTeamId(ctx *gin.Context)
	FindGameCoverByGameId(ctx *gin.Context)
	SetGameCoverByGameId(ctx *gin.Context)
	FindGameWriteUpByTeamId(ctx *gin.Context)
}
