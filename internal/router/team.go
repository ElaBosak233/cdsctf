package router

import (
	"github.com/elabosak233/cloudsdale/internal/controller"
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/service"
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
	t.router.POST("/", t.controller.Create)
	t.router.DELETE("/:id", t.CanModifyTeam(), t.controller.Delete)
	t.router.PUT("/:id", t.CanModifyTeam(), t.controller.Update)
	t.router.POST("/:id/users", t.controller.CreateUser)
	t.router.DELETE("/:id/users/:user_id", t.CanModifyTeam(), t.controller.DeleteUser)
	t.router.GET("/:id/invite", t.CanModifyTeam(), t.controller.GetInviteToken)
	t.router.PUT("/:id/invite", t.CanModifyTeam(), t.controller.UpdateInviteToken)
	t.router.POST("/:id/join", t.controller.Join)
	t.router.POST("/:id/leave", t.controller.Leave)
	t.router.POST("/:id/avatar", t.CanModifyTeam(), t.controller.SaveAvatar)
	t.router.DELETE("/:id/avatar", t.CanModifyTeam(), t.controller.DeleteAvatar)
}

func (t *TeamRouter) CanModifyTeam() gin.HandlerFunc {
	return func(ctx *gin.Context) {
		user := ctx.MustGet("user").(*model.User)

		if ok := service.S().AuthService.CanModifyTeam(user, convertor.ToUintD(ctx.Param("id"), 0)); !ok {
			ctx.JSON(http.StatusForbidden, gin.H{
				"code": http.StatusForbidden,
			})
			ctx.Abort()
		}

		ctx.Next()
	}
}
