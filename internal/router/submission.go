package router

import (
	"github.com/elabosak233/cloudsdale/internal/controller"
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/utils/convertor"
	"github.com/gin-gonic/gin"
)

type ISubmissionRouter interface {
	Register()
}

type SubmissionRouter struct {
	router     *gin.RouterGroup
	controller controller.ISubmissionController
}

func NewSubmissionRouter(submissionRouter *gin.RouterGroup, submissionController controller.ISubmissionController) ISubmissionRouter {
	return &SubmissionRouter{
		router:     submissionRouter,
		controller: submissionController,
	}
}

func (s *SubmissionRouter) Register() {
	s.router.GET("/", s.PreProcess(), s.controller.Find)
	s.router.POST("/", s.controller.Create)
	s.router.DELETE("/:id", s.controller.Delete)
}

func (s *SubmissionRouter) PreProcess() gin.HandlerFunc {
	return func(ctx *gin.Context) {
		if convertor.ToBoolD(ctx.Query("is_detailed"), false) {
			user := ctx.MustGet("user").(*model.User)
			if user.Group.Name == "admin" {
				ctx.Set("is_detailed", true)
			}
		} else {
			ctx.Set("is_detailed", false)
		}
		ctx.Next()
	}
}
