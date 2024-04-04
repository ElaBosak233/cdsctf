package router

import (
	"github.com/elabosak233/cloudsdale/internal/controller"
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/service"
	"github.com/elabosak233/cloudsdale/internal/utils/convertor"
	"github.com/gin-gonic/gin"
	"net/http"
)

type IUserRouter interface {
	Register()
}

type UserRouter struct {
	router     *gin.RouterGroup
	controller controller.IUserController
}

func NewUserRouter(userRouter *gin.RouterGroup, userController controller.IUserController) IUserRouter {
	return &UserRouter{
		router:     userRouter,
		controller: userController,
	}
}

func (u *UserRouter) Register() {
	u.router.GET("/", u.controller.Find)
	u.router.POST("/", u.controller.Create)
	u.router.PUT("/:id", u.CanModifyUser(), u.controller.Update)
	u.router.DELETE("/:id", u.CanModifyUser(), u.controller.Delete)
	u.router.POST("/login", u.controller.Login)
	u.router.POST("/logout", u.controller.Logout)
	u.router.POST("/register", u.controller.Register)
}

func (u *UserRouter) CanModifyUser() gin.HandlerFunc {
	return func(ctx *gin.Context) {
		user := ctx.MustGet("user").(*model.User)
		if ok := service.S().AuthService.CanModifyUser(user, convertor.ToUintD(ctx.Param("id"), 0)); !ok {
			ctx.JSON(http.StatusForbidden, gin.H{
				"code": http.StatusForbidden,
			})
			ctx.Abort()
		}
		ctx.Next()
	}
}
