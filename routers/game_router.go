package routers

import (
	"github.com/elabosak233/pgshub/controllers"
	"github.com/elabosak233/pgshub/middlewares"
	"github.com/gin-gonic/gin"
)

func NewGameRouter(gameRouter *gin.RouterGroup, gameController controllers.GameController, authMiddleware middlewares.AuthMiddleware) {
	gameRouter.GET("/", authMiddleware.Auth(), gameController.Find)
	gameRouter.GET("/:id/challenges", gameController.GetChallengesByGameId)
	gameRouter.GET("/:id/scoreboard", gameController.GetScoreboardByGameId)
	gameRouter.POST("/", authMiddleware.AuthInRole(3), gameController.Create)
	gameRouter.PUT("/", authMiddleware.AuthInRole(3), gameController.Update)
	gameRouter.DELETE("/", authMiddleware.AuthInRole(3), gameController.Delete)
}
