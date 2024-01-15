package routers

import (
	"github.com/elabosak233/pgshub/internal/controllers"
	"github.com/gin-gonic/gin"
)

func NewAssetRouter(assetRouter *gin.RouterGroup, assetController controllers.AssetController) {
	assetRouter.GET("/users/avatar", assetController.GetUserAvatarList)
	assetRouter.GET("/users/avatar/:id", assetController.FindUserAvatarByUserId)
	assetRouter.DELETE("/users/avatar/:id", assetController.DeleteUserAvatarByUserId)
	assetRouter.GET("/users/avatar/:id/exists", assetController.CheckUserAvatarExistsByUserId)
	assetRouter.POST("/users/avatar/:id", assetController.SetUserAvatarByUserId)
	assetRouter.GET("/teams/avatar", assetController.GetTeamAvatarList)
	assetRouter.GET("/teams/avatar/:id", assetController.FindTeamAvatarByTeamId)
	assetRouter.GET("/teams/avatar/:id/exists", assetController.CheckTeamAvatarExistsByTeamId)
	assetRouter.POST("/teams/avatar/:id", assetController.SetTeamAvatarByTeamId)
	assetRouter.GET("/games/cover/:id", assetController.FindGameCoverByGameId)
	assetRouter.POST("/games/cover/:id", assetController.SetGameCoverByGameId)
	assetRouter.GET("/games/writeups/:id", assetController.FindGameWriteUpByTeamId)
	assetRouter.GET("/challenges/attachments/:id", assetController.FindChallengeAttachmentByChallengeId)
	assetRouter.GET("/challenges/attachments/:id/exists", assetController.CheckChallengeAttachmentByChallengeId)
	assetRouter.POST("/challenges/attachments/:id", assetController.SetChallengeAttachmentByChallengeId)
	assetRouter.DELETE("/challenges/attachments/:id", assetController.DeleteChallengeAttachmentByChallengeId)
}
