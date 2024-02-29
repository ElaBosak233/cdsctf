package router

import (
	"github.com/elabosak233/cloudsdale/internal/controller"
	"github.com/gin-gonic/gin"
)

type IChallengeRouter interface {
	Register()
}

type ChallengeRouter struct {
	router     *gin.RouterGroup
	controller controller.IChallengeController
}

func NewChallengeRouter(challengeRouter *gin.RouterGroup, challengeController controller.IChallengeController) IChallengeRouter {
	return &ChallengeRouter{
		router:     challengeRouter,
		controller: challengeController,
	}
}

func (c *ChallengeRouter) Register() {
	c.router.GET("/", c.controller.Find)
	c.router.POST("/", c.controller.Create)
	c.router.PUT("/:id", c.controller.Update)
	c.router.DELETE("/:id", c.controller.Delete)
}
