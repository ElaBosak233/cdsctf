package router

import (
	"github.com/elabosak233/pgshub/internal/controller"
	"github.com/elabosak233/pgshub/internal/middleware"
	"github.com/gin-gonic/gin"
)

func NewSubmissionRouter(submissionRouter *gin.RouterGroup, submissionController controller.ISubmissionController, authMiddleware middleware.IAuthMiddleware) {
	submissionRouter.GET("/", authMiddleware.Auth(), submissionController.Find)
	submissionRouter.GET("/batch", authMiddleware.Auth(), submissionController.BatchFind)
	submissionRouter.POST("/", authMiddleware.Auth(), submissionController.Create)
	submissionRouter.DELETE("/", authMiddleware.AuthInRole(1), submissionController.Delete)
}
