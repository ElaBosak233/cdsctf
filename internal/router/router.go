package router

import (
	"github.com/elabosak233/cloudsdale/internal/controller"
	"github.com/gin-gonic/gin"
	"net/http"
)

func NewRouter(
	router *gin.RouterGroup,
	appController *controller.Controller,
) {
	router.GET("/", func(ctx *gin.Context) {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusOK,
			"msg":  "This is the heart of Cloudsdale.",
		})
	})
	NewUserRouter(router.Group("/users"), appController.UserController).Register()
	NewChallengeRouter(router.Group("/challenges"), appController.ChallengeController).Register()
	NewPodRouter(router.Group("/pods"), appController.InstanceController).Register()
	NewConfigRouter(router.Group("/configs"), appController.ConfigController).Register()
	NewMediaRouter(router.Group("/media"), appController.MediaController).Register()
	NewTeamRouter(router.Group("/teams"), appController.TeamController).Register()
	NewSubmissionRouter(router.Group("/submissions"), appController.SubmissionController).Register()
	NewGameRouter(router.Group("/games"), appController.GameController).Register()
	NewCategoryRouter(router.Group("/categories"), appController.CategoryController).Register()
	NewProxyRouter(router.Group("/proxies"), appController.ProxyController).Register()
}
