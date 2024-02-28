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

// Casbin
// The first layer of access control
// Default role is guest
// If the user is logged in, the role will be the user's group
// If the user's role has permission to access the resource, the request will be passed
// By the way, the user's information will be set to the context
func (m *CasbinMiddleware) Casbin() gin.HandlerFunc {
	return func(ctx *gin.Context) {
		var sub string
		var user response.UserResponse
		sub = "guest"

		token := ctx.GetHeader("Authorization")
		if token != "" {
			pgsToken, _ := jwt.Parse(token, func(token *jwt.Token) (interface{}, error) {
				return []byte(config.AppCfg().Gin.Jwt.SecretKey), nil
			})
			if claims, ok := pgsToken.Claims.(jwt.MapClaims); ok && pgsToken.Valid {
				userID := uint(claims["user_id"].(float64))
				user, _ = m.appService.UserService.FindById(userID)
				sub = user.Group.Name
			}
		}

		ok, _ := casbin.Enforcer.Enforce(sub, ctx.Request.URL.Path, ctx.Request.Method)
		if ok {
			ctx.Set("user", &user)
			ctx.Next()
			return
		}
		ctx.JSON(http.StatusUnauthorized, gin.H{
			"code": http.StatusUnauthorized,
			"sub":  sub,
			"msg":  "You have no permission to do that.",
		})
		ctx.Abort()
	}
}
