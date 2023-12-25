package routers

import (
	"github.com/elabosak233/pgshub/internal/controllers"
	"github.com/gin-gonic/gin"
)

func NewTeamRouter(teamRouter *gin.RouterGroup, teamController controllers.TeamController) {
	teamRouter.GET("/", teamController.Find)
	teamRouter.GET("/id/:id", teamController.FindById)
	teamRouter.POST("/", teamController.Create)
	teamRouter.DELETE("/", teamController.Delete)
	teamRouter.PUT("/", teamController.Update)
	teamRouter.POST("/members", teamController.Join)
	teamRouter.DELETE("/members", teamController.Quit)
}
