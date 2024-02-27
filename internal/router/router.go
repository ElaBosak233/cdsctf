package router

import (
	"github.com/elabosak233/cloudsdale/internal/controller"
	"github.com/elabosak233/cloudsdale/internal/middleware"
	"github.com/gin-gonic/gin"
	"net/http"
)

func NewRouter(
	router *gin.RouterGroup,
	appController *controller.Controller,
	appMiddleware *middleware.Middleware,
) {
	router.GET("/", appMiddleware.CasbinMiddleware.Casbin(), func(ctx *gin.Context) {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusOK,
			"msg":  "This is the heart of Cloudsdale.",
		})
	})
	NewUserRouter(router.Group("/users"), appController.UserController, appMiddleware.AuthMiddleware)
	NewChallengeRouter(router.Group("/challenges"), appController.ChallengeController, appMiddleware.AuthMiddleware)
	NewPodRouter(router.Group("/pods"), appController.InstanceController, appMiddleware.AuthMiddleware)
	NewConfigRouter(router.Group("/configs"), appController.ConfigController, appMiddleware.AuthMiddleware)
	NewMediaRouter(router.Group("/media"), appController.MediaController)
	NewTeamRouter(router.Group("/teams"), appController.TeamController)
	NewSubmissionRouter(router.Group("/submissions"), appController.SubmissionController, appMiddleware.AuthMiddleware)
	NewGameRouter(router.Group("/games"), appController.GameController, appMiddleware.AuthMiddleware)
	NewCategoryRouter(router.Group("/categories"), appController.CategoryController, appMiddleware.AuthMiddleware)
	NewProxyRouter(router.Group("/proxies"), appController.ProxyController)
}
