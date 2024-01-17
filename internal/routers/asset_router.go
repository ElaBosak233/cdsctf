package routers

import (
	"github.com/elabosak233/pgshub/internal/controllers"
	"github.com/gin-gonic/gin"
)

func NewAssetRouter(assetRouter *gin.RouterGroup, assetController controllers.AssetController) {
	assetRouter.GET("/users/avatar", assetController.GetUserAvatarList)
	assetRouter.GET("/users/avatar/:id", assetController.GetUserAvatarByUserId)
	assetRouter.DELETE("/users/avatar/:id", assetController.DeleteUserAvatarByUserId)
	assetRouter.GET("/users/avatar/:id/info", assetController.GetUserAvatarInfoByUserId)
	assetRouter.POST("/users/avatar/:id", assetController.SetUserAvatarByUserId)
	assetRouter.GET("/teams/avatar", assetController.GetTeamAvatarList)
	assetRouter.GET("/teams/avatar/:id", assetController.FindTeamAvatarByTeamId)
	assetRouter.GET("/teams/avatar/:id/info", assetController.CheckTeamAvatarExistsByTeamId)
	assetRouter.POST("/teams/avatar/:id", assetController.SetTeamAvatarByTeamId)
	assetRouter.GET("/games/cover/:id", assetController.FindGameCoverByGameId)
	assetRouter.POST("/games/cover/:id", assetController.SetGameCoverByGameId)
	assetRouter.GET("/games/writeups/:id", assetController.FindGameWriteUpByTeamId)
	assetRouter.GET("/challenges/attachments/:id", assetController.GetChallengeAttachmentByChallengeId)
	assetRouter.GET("/challenges/attachments/:id/info", assetController.GetChallengeAttachmentInfoByChallengeId)
	assetRouter.POST("/challenges/attachments/:id", assetController.SetChallengeAttachmentByChallengeId)
	assetRouter.DELETE("/challenges/attachments/:id", assetController.DeleteChallengeAttachmentByChallengeId)
}
