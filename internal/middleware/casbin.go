package middleware

import (
	"github.com/elabosak233/cloudsdale/internal/app/config"
	"github.com/elabosak233/cloudsdale/internal/extension/casbin"
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"github.com/elabosak233/cloudsdale/internal/service"
	"github.com/gin-gonic/gin"
	"github.com/golang-jwt/jwt/v5"
	"net/http"
)

// Casbin
// The first layer of access control
// Default role is guest
// If the user is logged in, the role will be the user's group
// If the user's role has permission to access the resource, the request will be passed
// By the way, the user's information will be set to the context
func Casbin() gin.HandlerFunc {

	appService := service.S()

	return func(ctx *gin.Context) {
		var sub string
		var user model.User
		sub = "guest"

		userToken := ctx.GetHeader("Authorization")
		if userToken != "" {
			pgsToken, _ := jwt.Parse(userToken, func(token *jwt.Token) (interface{}, error) {
				return []byte(config.JwtSecretKey()), nil
			})
			if claims, ok := pgsToken.Claims.(jwt.MapClaims); ok && pgsToken.Valid {
				if users, _, err := appService.UserService.Find(request.UserFindRequest{
					ID: uint(claims["user_id"].(float64)),
				}); err == nil && len(users) > 0 {
					user = users[0]
				}
				sub = user.Group
			}
		}

		ok, err := casbin.Enforcer.Enforce(sub, ctx.Request.URL.Path, ctx.Request.Method)
		if !ok || err != nil {
			switch sub {
			case "guest":
				ctx.JSON(http.StatusUnauthorized, gin.H{
					"code": http.StatusUnauthorized,
				})
			default:
				ctx.JSON(http.StatusForbidden, gin.H{
					"code": http.StatusForbidden,
				})
			}
			ctx.Abort()
		}
		ctx.Set("user", &user)
		ctx.Next()
		return
	}
}
