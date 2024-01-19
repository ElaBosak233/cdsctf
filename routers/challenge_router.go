package routers

import (
	"github.com/elabosak233/pgshub/controllers"
	"github.com/elabosak233/pgshub/middlewares"
	"github.com/gin-gonic/gin"
)

func NewChallengeRouter(challengeRouter *gin.RouterGroup, challengeController controllers.ChallengeController, authMiddleware middlewares.AuthMiddleware) {
	challengeRouter.GET("/", authMiddleware.Auth(), challengeController.Find)
	challengeRouter.POST("/", authMiddleware.AuthInRole(2), challengeController.Create)
	challengeRouter.PUT("/", authMiddleware.AuthInRole(2), challengeController.Update)
	challengeRouter.DELETE("/", authMiddleware.AuthInRole(2), challengeController.Delete)
}
