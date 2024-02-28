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
	NewUserRouter(router.Group("/users"), appController.UserController)
	NewChallengeRouter(router.Group("/challenges"), appController.ChallengeController)
	NewPodRouter(router.Group("/pods"), appController.InstanceController)
	NewConfigRouter(router.Group("/configs"), appController.ConfigController)
	NewMediaRouter(router.Group("/media"), appController.MediaController)
	NewTeamRouter(router.Group("/teams"), appController.TeamController)
	NewSubmissionRouter(router.Group("/submissions"), appController.SubmissionController)
	NewGameRouter(router.Group("/games"), appController.GameController)
	NewCategoryRouter(router.Group("/categories"), appController.CategoryController)
	NewProxyRouter(router.Group("/proxies"), appController.ProxyController)
}
