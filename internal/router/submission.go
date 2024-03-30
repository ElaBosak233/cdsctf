package router

import (
	"github.com/elabosak233/cloudsdale/internal/controller"
	"github.com/elabosak233/cloudsdale/internal/model/response"
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
	s.router.GET("/", s.SAuth(), s.controller.Find)
	s.router.POST("/", s.controller.Create)
	s.router.DELETE("/:id", s.controller.Delete)
}

func (s *SubmissionRouter) SAuth() gin.HandlerFunc {
	return func(ctx *gin.Context) {
		if convertor.ToBoolD(ctx.Query("is_detailed"), false) {
			user, _ := ctx.Get("user")
			if user.(*response.UserResponse).Group.Name == "admin" {
				ctx.Set("is_detailed", true)
			}
		} else {
			ctx.Set("is_detailed", false)
		}
		ctx.Next()
	}
}
