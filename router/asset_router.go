package router

import (
	"github.com/elabosak233/pgshub/controller"
	"github.com/gin-gonic/gin"
)

func NewAssetRouter(assetRouter *gin.RouterGroup, assetController controller.AssetController) {
	assetRouter.GET("/users/avatar/:id", assetController.FindUserAvatarByUserId)
	assetRouter.POST("/users/avatar/:id", assetController.SetUserAvatarByUserId)
	assetRouter.GET("/teams/avatar/:id", assetController.FindTeamAvatarByTeamId)
	assetRouter.POST("/teams/avatar/:id", assetController.SetTeamAvatarByTeamId)
	assetRouter.GET("/games/cover/:id", assetController.FindGameCoverByGameId)
	assetRouter.POST("/games/cover/:id", assetController.SetGameCoverByGameId)
	assetRouter.GET("/games/writeups/:id", assetController.FindGameWriteUpByTeamId)
}
