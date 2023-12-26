package routers

import (
	"github.com/elabosak233/pgshub/internal/controllers"
	"github.com/elabosak233/pgshub/internal/middlewares"
	"github.com/gin-gonic/gin"
)

func NewSubmissionRouter(submissionRouter *gin.RouterGroup, submissionController controllers.SubmissionController, authMiddleware middlewares.AuthMiddleware) {
	submissionRouter.GET("/", authMiddleware.Auth(), submissionController.Find)
}
