package router

import (
	"github.com/elabosak233/cloudsdale/internal/controller"
	"github.com/gin-gonic/gin"
)

type IGameRouter interface {
	Register()
}

type GameRouter struct {
	router     *gin.RouterGroup
	controller controller.IGameController
}

func NewGameRouter(gameRouter *gin.RouterGroup, gameController controller.IGameController) IGameRouter {
	return &GameRouter{
		router:     gameRouter,
		controller: gameController,
	}
}

func (g *GameRouter) Register() {
	g.router.GET("/", g.controller.Find)
	g.router.GET("/:id/challenges", g.controller.GetChallengesByGameId)
	g.router.GET("/:id/scoreboard", g.controller.GetScoreboardByGameId)
	g.router.GET("/:id/broadcast", g.controller.BroadCast)
	g.router.POST("/", g.controller.Create)
	g.router.PUT("/", g.controller.Update)
	g.router.DELETE("/", g.controller.Delete)
}
