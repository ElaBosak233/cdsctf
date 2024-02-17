package router

import (
	"github.com/elabosak233/cloudsdale/internal/controller"
	"github.com/elabosak233/cloudsdale/internal/middleware"
	"github.com/gin-gonic/gin"
)

func NewGameRouter(gameRouter *gin.RouterGroup, gameController controller.IGameController, authMiddleware middleware.IAuthMiddleware) {
	gameRouter.GET("/", authMiddleware.Auth(), gameController.Find)
	gameRouter.GET("/:id/challenges", gameController.GetChallengesByGameId)
	gameRouter.GET("/:id/scoreboard", gameController.GetScoreboardByGameId)
	gameRouter.GET("/:id/broadcast", gameController.BroadCast)
	gameRouter.POST("/", authMiddleware.AuthInRole(3), gameController.Create)
	gameRouter.PUT("/", authMiddleware.AuthInRole(3), gameController.Update)
	gameRouter.DELETE("/", authMiddleware.AuthInRole(3), gameController.Delete)
}
