package router

import (
	"github.com/elabosak233/cloudsdale/internal/controller"
	"github.com/elabosak233/cloudsdale/internal/model/response"
	"github.com/elabosak233/cloudsdale/internal/utils/convertor"
	"github.com/gin-gonic/gin"
	"net/http"
)

type IUserRouter interface {
	Register()
	SAuth() gin.HandlerFunc
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
	u.router.PUT("/:id", u.SAuth(), u.controller.Update)
	u.router.DELETE("/:id", u.SAuth(), u.controller.Delete)
	u.router.POST("/login", u.controller.Login)
	u.router.POST("/logout", u.controller.Logout)
	u.router.POST("/register", u.controller.Register)
	u.router.GET("/token/:token", u.controller.VerifyToken)
}

func (u *UserRouter) SAuth() gin.HandlerFunc {
	return func(ctx *gin.Context) {
		var user *response.UserResponse
		if u, ok := ctx.Get("user"); ok {
			user = u.(*response.UserResponse)
		}

		if user.Group.Name == "admin" || user.ID == convertor.ToUintD(ctx.Param("id"), 0) {
			ctx.Next()
			return
		}

		ctx.JSON(http.StatusUnauthorized, gin.H{
			"code": http.StatusUnauthorized,
		})
		ctx.Abort()
	}
}
