package router

import (
	"github.com/elabosak233/pgshub/controller"
	"github.com/gin-gonic/gin"
)

func NewTeamRouter(teamRouter *gin.RouterGroup, teamController controller.TeamController) {
	teamRouter.GET("/", teamController.FindAll)
	teamRouter.GET("/id/:id", teamController.FindById)
	teamRouter.POST("/", teamController.Create)
	teamRouter.DELETE("/", teamController.Delete)
	teamRouter.PUT("/", teamController.Update)
	teamRouter.POST("/members", teamController.Join)
	teamRouter.DELETE("/members", teamController.Quit)
}
