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
	NewUserRouter(router.Group("/users"), appController.UserController, appMiddleware.AuthMiddleware)
	NewChallengeRouter(router.Group("/challenges"), appController.ChallengeController, appMiddleware.AuthMiddleware)
	NewInstanceRouter(router.Group("/instances"), appController.InstanceController, appMiddleware.AuthMiddleware)
	NewConfigRouter(router.Group("/configs"), appController.ConfigController, appMiddleware.AuthMiddleware)
	NewAssetRouter(router.Group("/assets"), appController.AssetController)
	NewTeamRouter(router.Group("/teams"), appController.TeamController)
	NewSubmissionRouter(router.Group("/submissions"), appController.SubmissionController, appMiddleware.AuthMiddleware)
}
