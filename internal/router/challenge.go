package router

import (
	"github.com/elabosak233/cloudsdale/internal/controller"
	"github.com/elabosak233/cloudsdale/internal/model/dto/response"
	"github.com/elabosak233/cloudsdale/internal/utils/convertor"
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
	c.router.GET("/", c.SAuth(), c.controller.Find)
	c.router.POST("/", c.controller.Create)
	c.router.PUT("/:id", c.controller.Update)
	c.router.DELETE("/:id", c.controller.Delete)
}

func (c *ChallengeRouter) SAuth() gin.HandlerFunc {
	return func(ctx *gin.Context) {
		user, _ := ctx.Get("user")
		if user.(*response.UserResponse).Group.Name == "admin" || user.(*response.UserResponse).Group.Name == "monitor" {
			if convertor.ToBoolD(ctx.Query("is_detailed"), false) {
				ctx.Set("is_detailed", true)
			}
		} else {
			ctx.Set("is_detailed", false)
		}
		if user.(*response.UserResponse).Group.Name == "admin" || user.(*response.UserResponse).Group.Name == "monitor" {
			if convertor.ToBoolD(ctx.Query("is_practicable"), false) {
				ctx.Set("is_practicable", true)
			}
		} else {
			ctx.Set("is_practicable", false)
		}
		ctx.Next()
	}
}
