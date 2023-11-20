package router

import (
	"github.com/elabosak233/pgshub/controller"
	"github.com/gin-gonic/gin"
)

func NewChallengeRouter(challengeRouter *gin.RouterGroup, challengeController *controller.ChallengeController) {
	challengeRouter.GET("/", challengeController.FindAll)
	challengeRouter.GET("/:id", challengeController.FindById)
	challengeRouter.POST("/", challengeController.Create)
	challengeRouter.PATCH("/", challengeController.Update)
	challengeRouter.DELETE("/", challengeController.Delete)
}
