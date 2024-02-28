package router

import (
	"github.com/elabosak233/cloudsdale/internal/controller"
	"github.com/gin-gonic/gin"
)

func NewSubmissionRouter(submissionRouter *gin.RouterGroup, submissionController controller.ISubmissionController) {
	submissionRouter.GET("/", submissionController.Find)
	submissionRouter.POST("/", submissionController.Create)
	submissionRouter.DELETE("/", submissionController.Delete)
}
