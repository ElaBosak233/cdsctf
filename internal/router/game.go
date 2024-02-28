package router

import (
	"github.com/elabosak233/cloudsdale/internal/controller"
	"github.com/gin-gonic/gin"
)

func NewGameRouter(gameRouter *gin.RouterGroup, gameController controller.IGameController) {
	gameRouter.GET("/", gameController.Find)
	gameRouter.GET("/:id/challenges", gameController.GetChallengesByGameId)
	gameRouter.GET("/:id/scoreboard", gameController.GetScoreboardByGameId)
	gameRouter.GET("/:id/broadcast", gameController.BroadCast)
	gameRouter.POST("/", gameController.Create)
	gameRouter.PUT("/", gameController.Update)
	gameRouter.DELETE("/", gameController.Delete)
}
