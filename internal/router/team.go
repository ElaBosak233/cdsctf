package router

import (
	"github.com/elabosak233/cloudsdale/internal/controller"
	"github.com/elabosak233/cloudsdale/internal/model/dto/response"
	"github.com/elabosak233/cloudsdale/internal/utils/convertor"
	"github.com/gin-gonic/gin"
	"net/http"
)

func NewTeamRouter(teamRouter *gin.RouterGroup, teamController controller.ITeamController) {
	teamRouter.GET("/", teamController.Find)
	teamRouter.GET("/:id", teamController.FindById)
	teamRouter.POST("/", teamController.Create)
	teamRouter.DELETE("/:id", TeamSAuth(), teamController.Delete)
	teamRouter.PUT("/:id", TeamSAuth(), teamController.Update)
	teamRouter.POST("/members", teamController.Join)
	teamRouter.DELETE("/members", teamController.Quit)
}

func TeamSAuth() gin.HandlerFunc {
	return func(ctx *gin.Context) {
		var user *response.UserResponse
		if u, ok := ctx.Get("user"); ok {
			user = u.(*response.UserResponse)
		}

		isCaptain := func() bool {
			for _, team := range user.Teams {
				if team.ID == convertor.ToUintD(ctx.Param("id"), 0) && team.CaptainId == user.ID {
					return true
				}
			}
			return false
		}

		if user.Group.Name == "admin" || isCaptain() {
			ctx.Next()
			return
		}

		ctx.JSON(http.StatusUnauthorized, gin.H{
			"code": http.StatusUnauthorized,
			"msg":  "You have no permission to do that.",
		})
		ctx.Abort()
	}
}
