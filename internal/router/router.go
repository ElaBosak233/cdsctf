package router

import (
	"fmt"
	"github.com/elabosak233/pgshub/internal/controller"
	"github.com/elabosak233/pgshub/internal/middleware"
	"github.com/gin-gonic/gin"
	"net/http"
)

func NewRouter(
	router *gin.RouterGroup,
	appController *controller.Controller,
	appMiddleware *middleware.Middleware,
) {
	router.GET("/", func(ctx *gin.Context) {
		ctx.String(http.StatusOK, fmt.Sprintf("The Backend of PgsHub"))
	})
	NewUserRouter(router.Group("/users"), appController.UserController, appMiddleware.AuthMiddleware)
	NewChallengeRouter(router.Group("/challenges"), appController.ChallengeController, appMiddleware.AuthMiddleware)
	NewPodRouter(router.Group("/pods"), appController.InstanceController, appMiddleware.AuthMiddleware)
	NewConfigRouter(router.Group("/configs"), appController.ConfigController, appMiddleware.AuthMiddleware)
	NewMediaRouter(router.Group("/media"), appController.MediaController)
	NewTeamRouter(router.Group("/teams"), appController.TeamController)
	NewSubmissionRouter(router.Group("/submissions"), appController.SubmissionController, appMiddleware.AuthMiddleware)
	NewGameRouter(router.Group("/games"), appController.GameController, appMiddleware.AuthMiddleware)
}
