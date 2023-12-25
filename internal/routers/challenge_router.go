package routers

import (
	"github.com/elabosak233/pgshub/internal/controllers"
	"github.com/gin-gonic/gin"
)

func NewChallengeRouter(challengeRouter *gin.RouterGroup, challengeController controllers.ChallengeController) {
	// 管理员
	challengeRouter.GET("/", challengeController.Find)
	challengeRouter.GET("/:id", challengeController.FindById)
	challengeRouter.POST("/", challengeController.Create)
	challengeRouter.PATCH("/", challengeController.Update)
	challengeRouter.DELETE("/", challengeController.Delete)
}
