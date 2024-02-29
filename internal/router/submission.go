package router

import (
	"github.com/elabosak233/cloudsdale/internal/controller"
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
	s.router.GET("/", s.controller.Find)
	s.router.POST("/", s.controller.Create)
	s.router.DELETE("/", s.controller.Delete)
}
