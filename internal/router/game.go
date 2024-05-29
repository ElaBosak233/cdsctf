package router

import (
	"github.com/elabosak233/cloudsdale/internal/controller"
	"github.com/elabosak233/cloudsdale/internal/model"
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
	g.router.GET("/", g.PreProcess(), g.controller.Find)
	g.router.POST("/", g.controller.Create)
	g.router.PUT("/:id", g.controller.Update)
	g.router.DELETE("/:id", g.controller.Delete)
	g.router.GET("/:id/challenges", g.controller.FindChallenge)
	g.router.POST("/:id/challenges", g.controller.CreateChallenge)
	g.router.PUT("/:id/challenges/:challenge_id", g.controller.UpdateChallenge)
	g.router.DELETE("/:id/challenges/:challenge_id", g.controller.DeleteChallenge)
	g.router.GET("/:id/teams", g.controller.FindTeam)
	g.router.GET("/:id/teams/:team_id", g.controller.FindTeamByID)
	g.router.POST("/:id/teams", g.controller.CreateTeam)
	g.router.PUT("/:id/teams/:team_id", g.controller.UpdateTeam)
	g.router.DELETE("/:id/teams/:team_id", g.controller.DeleteTeam)
	g.router.GET("/:id/notices", g.controller.FindNotice)
	g.router.POST("/:id/notices", g.controller.CreateNotice)
	g.router.PUT("/:id/notices/:notice_id", g.controller.UpdateNotice)
	g.router.DELETE("/:id/notices/:notice_id", g.controller.DeleteNotice)
	g.router.GET("/:id/broadcast", g.controller.BroadCast)
	g.router.POST("/:id/poster", g.controller.SavePoster)
	g.router.DELETE("/:id/poster", g.controller.DeletePoster)
}

func (g *GameRouter) PreProcess() gin.HandlerFunc {
	return func(ctx *gin.Context) {
		user := ctx.MustGet("user").(*model.User)
		if !(user.Group == "admin") {
			ctx.Set("is_enabled", true)
		}
		ctx.Next()
	}
}
