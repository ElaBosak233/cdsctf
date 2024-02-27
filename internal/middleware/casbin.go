package middleware

import (
	"github.com/elabosak233/cloudsdale/internal/casbin"
	"github.com/elabosak233/cloudsdale/internal/config"
	"github.com/elabosak233/cloudsdale/internal/model/dto/response"
	"github.com/elabosak233/cloudsdale/internal/service"
	"github.com/gin-gonic/gin"
	"github.com/golang-jwt/jwt/v5"
	"net/http"
)

type ICasbinMiddleware interface {
	Casbin() gin.HandlerFunc
}

type CasbinMiddleware struct {
	appService *service.Service
}

func NewCasbinMiddleware(appService *service.Service) ICasbinMiddleware {
	return &CasbinMiddleware{
		appService: appService,
	}
}

func (m *CasbinMiddleware) Casbin() gin.HandlerFunc {
	return func(ctx *gin.Context) {
		token := ctx.GetHeader("Authorization")
		pgsToken, err := jwt.Parse(token, func(token *jwt.Token) (interface{}, error) {
			return []byte(config.AppCfg().Gin.Jwt.SecretKey), nil
		})
		if err != nil {
			ctx.JSON(http.StatusOK, gin.H{
				"code": http.StatusUnauthorized,
				"msg":  err.Error(),
			})
			ctx.Abort()
			return
		}
		var user response.UserResponse
		if claims, ok := pgsToken.Claims.(jwt.MapClaims); ok && pgsToken.Valid {
			userId := uint(claims["user_id"].(float64))
			ctx.Set("UserID", userId)
			user, err = m.appService.UserService.FindById(userId)
			if err != nil {
				ctx.JSON(http.StatusOK, gin.H{
					"code": http.StatusUnauthorized,
					"msg":  "无效 Token",
				})
				ctx.Abort()
				return
			}
			ctx.Set("UserGroupID", user.Group.ID)
		} else {
			ctx.JSON(http.StatusOK, gin.H{
				"code": http.StatusUnauthorized,
				"msg":  "无效 Token",
			})
			ctx.Abort()
			return
		}
		ok, err := casbin.Enforcer.Enforce(user.Group.Name, ctx.Request.URL.Path, ctx.Request.Method)
		if ok {
			ctx.Next()
			return
		}
		ctx.JSON(http.StatusUnauthorized, gin.H{
			"code": http.StatusUnauthorized,
		})
		ctx.Abort()
	}
}
