package router

import (
	"github.com/elabosak233/cloudsdale/internal/controller"
	"github.com/elabosak233/cloudsdale/internal/model/response"
	"github.com/elabosak233/cloudsdale/internal/utils/convertor"
	"github.com/gin-gonic/gin"
	"net/http"
)

type ITeamRouter interface {
	Register()
}

type TeamRouter struct {
	router     *gin.RouterGroup
	controller controller.ITeamController
}

func NewTeamRouter(teamRouter *gin.RouterGroup, teamController controller.ITeamController) ITeamRouter {
	return &TeamRouter{
		router:     teamRouter,
		controller: teamController,
	}
}

func (t *TeamRouter) Register() {
	t.router.GET("/", t.controller.Find)
	t.router.GET("/:id", t.controller.FindById)
	t.router.POST("/", t.controller.Create)
	t.router.DELETE("/:id", t.SAuth(), t.controller.Delete)
	t.router.PUT("/:id", t.SAuth(), t.controller.Update)
	t.router.POST("/:id/users", t.controller.CreateUser)
	t.router.DELETE("/:id/users/:user_id", t.controller.DeleteUser)
}

func (t *TeamRouter) SAuth() gin.HandlerFunc {
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
		})
		ctx.Abort()
	}
}
