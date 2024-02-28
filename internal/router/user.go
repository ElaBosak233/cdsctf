package router

import (
	"github.com/elabosak233/cloudsdale/internal/controller"
	"github.com/elabosak233/cloudsdale/internal/model/dto/response"
	"github.com/elabosak233/cloudsdale/internal/utils/convertor"
	"github.com/gin-gonic/gin"
	"net/http"
)

func NewUserRouter(userRouter *gin.RouterGroup, userController controller.IUserController) {
	userRouter.GET("/", userController.Find)
	userRouter.POST("/", userController.Create)
	userRouter.PUT("/:id", UserSAuth(), userController.Update)
	userRouter.DELETE("/:id", UserSAuth(), userController.Delete)
	userRouter.POST("/login", userController.Login)
	userRouter.POST("/logout", userController.Logout)
	userRouter.POST("/register", userController.Register)
	userRouter.GET("/token/:token", userController.VerifyToken)
}

func UserSAuth() gin.HandlerFunc {
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
			"msg":  "You have no permission to do that.",
		})
		ctx.Abort()
	}
}
