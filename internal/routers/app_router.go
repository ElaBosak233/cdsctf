package routers

import (
	"fmt"
	"github.com/elabosak233/pgshub/internal/controllers"
	"github.com/elabosak233/pgshub/internal/middlewares"
	"github.com/gin-gonic/gin"
	"net/http"
)

func NewRouters(
	router *gin.RouterGroup,
	appController *controllers.AppController,
	appMiddleware *middlewares.AppMiddleware,
) {
	router.GET("/", func(ctx *gin.Context) {
		ctx.String(http.StatusOK, fmt.Sprintf("The Backend of PgsHub"))
	})
	userRouter := router.Group("/users")
	NewUserRouter(userRouter, appController.UserController, appMiddleware.AuthMiddleware)
	challengeRouter := router.Group("/challenges")
	NewChallengeRouter(challengeRouter, appController.ChallengeController)
	instanceRouter := router.Group("/instances")
	NewInstanceRouter(instanceRouter, appController.InstanceController)
	configRouter := router.Group("/configs")
	NewConfigRouter(configRouter, appController.ConfigController)
	assetRouter := router.Group("/assets")
	NewAssetRouter(assetRouter, appController.AssetController)
	teamRouter := router.Group("/teams")
	NewTeamRouter(teamRouter, appController.TeamController)
}
