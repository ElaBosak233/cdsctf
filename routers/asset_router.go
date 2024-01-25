package routers

import (
	"github.com/elabosak233/pgshub/controllers"
	"github.com/gin-gonic/gin"
)

func NewAssetRouter(assetRouter *gin.RouterGroup, assetController controllers.AssetController) {
	assetRouter.GET("/users/avatar", assetController.GetUserAvatarList)
	assetRouter.GET("/users/avatar/:id", assetController.GetUserAvatarByUserId)
	assetRouter.DELETE("/users/avatar/:id", assetController.DeleteUserAvatarByUserId)
	assetRouter.GET("/users/avatar/:id/info", assetController.GetUserAvatarInfoByUserId)
	assetRouter.POST("/users/avatar/:id", assetController.SetUserAvatarByUserId)
	assetRouter.GET("/teams/avatar", assetController.GetTeamAvatarList)
	assetRouter.GET("/teams/avatar/:id", assetController.GetTeamAvatarByTeamId)
	assetRouter.GET("/teams/avatar/:id/info", assetController.GetTeamAvatarInfoByTeamId)
	assetRouter.POST("/teams/avatar/:id", assetController.SetTeamAvatarByTeamId)
	assetRouter.DELETE("/teams/avatar/:id", assetController.DeleteTeamAvatarByTeamId)
	assetRouter.GET("/games/cover/:id", assetController.GetGameCoverByGameId)
	assetRouter.POST("/games/cover/:id", assetController.SetGameCoverByGameId)
	assetRouter.GET("/games/writeups/:id", assetController.FindGameWriteUpByTeamId)
	assetRouter.GET("/challenges/attachments/:id", assetController.GetChallengeAttachmentByChallengeId)
	assetRouter.GET("/challenges/attachments/:id/info", assetController.GetChallengeAttachmentInfoByChallengeId)
	assetRouter.POST("/challenges/attachments/:id", assetController.SetChallengeAttachmentByChallengeId)
	assetRouter.DELETE("/challenges/attachments/:id", assetController.DeleteChallengeAttachmentByChallengeId)
}
