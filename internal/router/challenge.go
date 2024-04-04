package router

import (
	"github.com/elabosak233/cloudsdale/internal/controller"
	"github.com/elabosak233/cloudsdale/internal/model"
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
	c.router.GET("/", c.PreProcess(), c.controller.Find)
	c.router.POST("/", c.controller.Create)
	c.router.PUT("/:id", c.controller.Update)
	c.router.DELETE("/:id", c.controller.Delete)
	c.router.POST("/:id/images", c.controller.CreateImage)
	c.router.PUT("/:id/images/:image_id", c.controller.UpdateImage)
	c.router.DELETE("/:id/images/:image_id", c.controller.DeleteImage)
	c.router.POST("/:id/hints", c.controller.CreateHint)
	c.router.PUT("/:id/hints/:hint_id", c.controller.UpdateHint)
	c.router.DELETE("/:id/hints/:hint_id", c.controller.DeleteHint)
	c.router.POST("/:id/flags", c.controller.CreateFlag)
	c.router.PUT("/:id/flags/:flag_id", c.controller.UpdateFlag)
	c.router.DELETE("/:id/flags/:flag_id", c.controller.DeleteFlag)
}

func (c *ChallengeRouter) PreProcess() gin.HandlerFunc {
	return func(ctx *gin.Context) {
		user := ctx.MustGet("user").(*model.User)
		if user.Group.Name == "admin" {
			ctx.Set("is_detailed", convertor.ToBoolD(ctx.Query("is_detailed"), false))
		} else {
			ctx.Set("is_detailed", false)
		}
		if user.Group.Name == "admin" {
			ctx.Set("is_practicable", convertor.ToBoolP(ctx.Query("is_practicable")))
		} else {
			ctx.Set("is_practicable", convertor.TrueP())
		}
		ctx.Next()
	}
}
