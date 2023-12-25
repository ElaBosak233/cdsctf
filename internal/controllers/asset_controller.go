package controllers

import "github.com/gin-gonic/gin"

type AssetController interface {
	GetUserAvatarList(ctx *gin.Context)
	FindUserAvatarByUserId(ctx *gin.Context)
	CheckUserAvatarExistsByUserId(ctx *gin.Context)
	SetUserAvatarByUserId(ctx *gin.Context)
	DeleteUserAvatarByUserId(ctx *gin.Context)
	GetTeamAvatarList(ctx *gin.Context)
	FindTeamAvatarByTeamId(ctx *gin.Context)
	CheckTeamAvatarExistsByTeamId(ctx *gin.Context)
	SetTeamAvatarByTeamId(ctx *gin.Context)
	FindGameCoverByGameId(ctx *gin.Context)
	SetGameCoverByGameId(ctx *gin.Context)
	FindGameWriteUpByTeamId(ctx *gin.Context)
}
