package router

import (
	"github.com/elabosak233/cloudsdale/internal/controller"
	"github.com/gin-gonic/gin"
)

func NewChallengeRouter(challengeRouter *gin.RouterGroup, challengeController controller.IChallengeController) {
	challengeRouter.GET("/", challengeController.Find)
	challengeRouter.POST("/", challengeController.Create)
	challengeRouter.PUT("/:id", challengeController.Update)
	challengeRouter.DELETE("/:id", challengeController.Delete)
}
