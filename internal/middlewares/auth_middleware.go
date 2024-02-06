package middlewares

import (
	"github.com/elabosak233/pgshub/internal/services"
	"github.com/gin-gonic/gin"
	jwt "github.com/golang-jwt/jwt/v5"
	"github.com/spf13/viper"
	"net/http"
)

type AuthMiddleware interface {
	Auth() gin.HandlerFunc
	AuthInRole(role int64) gin.HandlerFunc
}

type AuthMiddlewareImpl struct {
	appService *services.AppService
}

func NewAuthMiddleware(appService *services.AppService) AuthMiddleware {
	return &AuthMiddlewareImpl{
		appService: appService,
	}
}

func (m *AuthMiddlewareImpl) BasicAuth(ctx *gin.Context) {
	token := ctx.GetHeader("PgsToken")
	pgsToken, err := jwt.Parse(token, func(token *jwt.Token) (interface{}, error) {
		return []byte(viper.GetString("jwt.secret_key")), nil
	})
	if err != nil {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusUnauthorized,
			"msg":  err.Error(),
		})
		ctx.Abort()
		return
	}
	if claims, ok := pgsToken.Claims.(jwt.MapClaims); ok && pgsToken.Valid {
		userId := claims["user_id"].(string)
		ctx.Set("UserId", userId)
		user, err := m.appService.UserService.FindById(userId)
		if err != nil {
			ctx.JSON(http.StatusOK, gin.H{
				"code": http.StatusUnauthorized,
				"msg":  "无效 Token",
			})
			ctx.Abort()
			return
		}
		ctx.Set("UserRole", user.Role)
	} else {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusUnauthorized,
			"msg":  "无效 Token",
		})
		ctx.Abort()
		return
	}
}

func (m *AuthMiddlewareImpl) AuthInRole(role int64) gin.HandlerFunc {
	return func(ctx *gin.Context) {
		m.BasicAuth(ctx)
		if ctx.GetInt64("UserRole") > role {
			ctx.JSON(http.StatusOK, gin.H{
				"code": http.StatusForbidden,
				"msg":  "权限不足",
			})
			ctx.Abort()
			return
		}
		ctx.Next()
	}
}

func (m *AuthMiddlewareImpl) Auth() gin.HandlerFunc {
	return func(ctx *gin.Context) {
		m.BasicAuth(ctx)
		ctx.Next()
	}
}
