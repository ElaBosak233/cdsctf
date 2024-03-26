package router

import (
	"github.com/elabosak233/cloudsdale/internal/controller"
	"github.com/gin-gonic/gin"
)

type IMediaRouter interface {
	Register()
}

type MediaRouter struct {
	router     *gin.RouterGroup
	controller controller.IMediaController
}

func NewMediaRouter(mediaRouter *gin.RouterGroup, mediaController controller.IMediaController) IMediaRouter {
	return &MediaRouter{
		router:     mediaRouter,
		controller: mediaController,
	}
}

func (m *MediaRouter) Register() {
	m.router.GET("/games/cover/:id", m.controller.GetGameCoverByGameId)
	m.router.POST("/games/cover/:id", m.controller.SetGameCoverByGameId)
	m.router.GET("/games/writeups/:id", m.controller.FindGameWriteUpByTeamId)
	m.router.GET("/challenges/attachments/:id", m.controller.GetChallengeAttachmentByChallengeId)
	m.router.GET("/challenges/attachments/:id/info", m.controller.GetChallengeAttachmentInfoByChallengeId)
	m.router.POST("/challenges/attachments/:id", m.controller.SetChallengeAttachmentByChallengeId)
	m.router.DELETE("/challenges/attachments/:id", m.controller.DeleteChallengeAttachmentByChallengeId)
}
