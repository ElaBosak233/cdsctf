package middleware

import (
	"fmt"
	"github.com/elabosak233/pgshub/internal/config"
	"github.com/elabosak233/pgshub/internal/service"
	"github.com/gin-gonic/gin"
	"github.com/golang-jwt/jwt/v5"
	"net/http"
)

type IAuthMiddleware interface {
	Auth() gin.HandlerFunc
	AuthInRole(role int64) gin.HandlerFunc
}

type AuthMiddleware struct {
	appService *service.Service
}

func NewAuthMiddleware(appService *service.Service) IAuthMiddleware {
	return &AuthMiddleware{
		appService: appService,
	}
}

func (m *AuthMiddleware) BasicAuth(ctx *gin.Context) {
	token := ctx.GetHeader("Authorization")
	pgsToken, err := jwt.Parse(token, func(token *jwt.Token) (interface{}, error) {
		return []byte(config.AppCfg().Jwt.SecretKey), nil
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
		userId := uint(claims["user_id"].(float64))
		ctx.Set("UserID", userId)
		user, err := m.appService.UserService.FindById(userId)
		if err != nil {
			ctx.JSON(http.StatusOK, gin.H{
				"code": http.StatusUnauthorized,
				"msg":  "无效 Token",
			})
			ctx.Abort()
			return
		}
		fmt.Println(user)
		ctx.Set("UserGroupID", user.Group.ID)
		ctx.Set("UserLevel", user.Group.Level)
	} else {
		ctx.JSON(http.StatusOK, gin.H{
			"code": http.StatusUnauthorized,
			"msg":  "无效 Token",
		})
		ctx.Abort()
		return
	}
}

func (m *AuthMiddleware) AuthInRole(role int64) gin.HandlerFunc {
	return func(ctx *gin.Context) {
		m.BasicAuth(ctx)
		if ctx.GetInt64("UserLevel") > role {
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

func (m *AuthMiddleware) Auth() gin.HandlerFunc {
	return func(ctx *gin.Context) {
		m.BasicAuth(ctx)
		ctx.Next()
	}
}
