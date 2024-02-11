package routers

import (
	"github.com/elabosak233/pgshub/controllers"
	"github.com/gin-gonic/gin"
)

func NewMediaRouter(mediaRouter *gin.RouterGroup, mediaController controllers.MediaController) {
	mediaRouter.GET("/users/avatar", mediaController.GetUserAvatarList)
	mediaRouter.GET("/users/avatar/:id", mediaController.GetUserAvatarByUserId)
	mediaRouter.DELETE("/users/avatar/:id", mediaController.DeleteUserAvatarByUserId)
	mediaRouter.GET("/users/avatar/:id/info", mediaController.GetUserAvatarInfoByUserId)
	mediaRouter.POST("/users/avatar/:id", mediaController.SetUserAvatarByUserId)
	mediaRouter.GET("/teams/avatar", mediaController.GetTeamAvatarList)
	mediaRouter.GET("/teams/avatar/:id", mediaController.GetTeamAvatarByTeamId)
	mediaRouter.GET("/teams/avatar/:id/info", mediaController.GetTeamAvatarInfoByTeamId)
	mediaRouter.POST("/teams/avatar/:id", mediaController.SetTeamAvatarByTeamId)
	mediaRouter.DELETE("/teams/avatar/:id", mediaController.DeleteTeamAvatarByTeamId)
	mediaRouter.GET("/games/cover/:id", mediaController.GetGameCoverByGameId)
	mediaRouter.POST("/games/cover/:id", mediaController.SetGameCoverByGameId)
	mediaRouter.GET("/games/writeups/:id", mediaController.FindGameWriteUpByTeamId)
	mediaRouter.GET("/challenges/attachments/:id", mediaController.GetChallengeAttachmentByChallengeId)
	mediaRouter.GET("/challenges/attachments/:id/info", mediaController.GetChallengeAttachmentInfoByChallengeId)
	mediaRouter.POST("/challenges/attachments/:id", mediaController.SetChallengeAttachmentByChallengeId)
	mediaRouter.DELETE("/challenges/attachments/:id", mediaController.DeleteChallengeAttachmentByChallengeId)
}
