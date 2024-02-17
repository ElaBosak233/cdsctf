package router

import (
	"github.com/elabosak233/cloudsdale/internal/controller"
	"github.com/elabosak233/cloudsdale/internal/middleware"
	"github.com/gin-gonic/gin"
)

func NewChallengeRouter(challengeRouter *gin.RouterGroup, challengeController controller.IChallengeController, authMiddleware middleware.IAuthMiddleware) {
	challengeRouter.GET("/", authMiddleware.Auth(), challengeController.Find)
	challengeRouter.POST("/", authMiddleware.AuthInRole(2), challengeController.Create)
	challengeRouter.PUT("/", authMiddleware.AuthInRole(2), challengeController.Update)
	challengeRouter.DELETE("/", authMiddleware.AuthInRole(2), challengeController.Delete)
}
