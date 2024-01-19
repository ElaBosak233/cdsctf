package routers

import (
	"github.com/elabosak233/pgshub/controllers"
	"github.com/elabosak233/pgshub/middlewares"
	"github.com/gin-gonic/gin"
)

func NewSubmissionRouter(submissionRouter *gin.RouterGroup, submissionController controllers.SubmissionController, authMiddleware middlewares.AuthMiddleware) {
	submissionRouter.GET("/", authMiddleware.Auth(), submissionController.Find)
	submissionRouter.GET("/batch", authMiddleware.Auth(), submissionController.BatchFind)
	submissionRouter.POST("/", authMiddleware.Auth(), submissionController.Create)
	submissionRouter.DELETE("/", authMiddleware.AuthInRole(1), submissionController.Delete)
}
