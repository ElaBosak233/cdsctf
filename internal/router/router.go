package router

import (
	"github.com/elabosak233/cloudsdale/internal/controller"
	ginI18n "github.com/gin-contrib/i18n"
	"github.com/gin-gonic/gin"
	"net/http"
)

var (
	r *Router = nil
)

type Router struct {
	router     *gin.RouterGroup
	controller *controller.Controller
}

func InitRouter(
	router *gin.RouterGroup,
) {
	r = &Router{
		router:     router,
		controller: controller.C(),
	}

	r.router.GET("/", func(ctx *gin.Context) {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusOK,
			"msg":  ginI18n.MustGetMessage(ctx, "welcome"),
		})
	})

	NewUserRouter(r.router.Group("/users"), r.controller.UserController).Register()
	NewChallengeRouter(r.router.Group("/challenges"), r.controller.ChallengeController).Register()
	NewPodRouter(r.router.Group("/pods"), r.controller.PodController).Register()
	NewConfigRouter(r.router.Group("/configs"), r.controller.ConfigController).Register()
	NewMediaRouter(r.router.Group("/media"), r.controller.MediaController).Register()
	NewTeamRouter(r.router.Group("/teams"), r.controller.TeamController).Register()
	NewSubmissionRouter(r.router.Group("/submissions"), r.controller.SubmissionController).Register()
	NewGameRouter(r.router.Group("/games"), r.controller.GameController).Register()
	NewCategoryRouter(r.router.Group("/categories"), r.controller.CategoryController).Register()
	NewProxyRouter(r.router.Group("/proxies"), r.controller.ProxyController).Register()
}
