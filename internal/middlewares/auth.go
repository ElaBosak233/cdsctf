package middlewares

import (
	"errors"
	"github.com/dgrijalva/jwt-go"
	"github.com/elabosak233/pgshub/internal/models/misc"
	"github.com/gin-gonic/gin"
	"github.com/spf13/viper"
	"net/http"
)

func authMiddleware(ctx *gin.Context) {
	tokenString := ctx.GetHeader("Authorization")
	if tokenString == "" {
		ctx.JSON(http.StatusUnauthorized, gin.H{
			"code":  http.StatusUnauthorized,
			"error": "缺少 Token",
		})
		ctx.Abort()
		return
	}
	claims := &misc.Claims{}
	token, err := jwt.ParseWithClaims(tokenString, claims, func(token *jwt.Token) (interface{}, error) {
		return viper.GetString("Jwt.SecretKey"), nil
	})
	if err != nil {
		if errors.Is(err, jwt.ErrSignatureInvalid) {
			ctx.JSON(http.StatusUnauthorized, gin.H{
				"code":  http.StatusUnauthorized,
				"error": "无效 Token",
			})
			ctx.Abort()
			return
		}
		ctx.JSON(http.StatusUnauthorized, gin.H{"error": "Unauthorized"})
		ctx.Abort()
		return
	}
	if claims, ok := token.Claims.(*misc.Claims); ok && token.Valid {
		ctx.Set("user_id", claims.UserId)
		ctx.Set("role", claims.Role)
		ctx.Next()
	} else {
		ctx.JSON(http.StatusUnauthorized, gin.H{
			"code":  http.StatusUnauthorized,
			"error": "无效 Token",
		})
		ctx.Abort()
		return
	}
}
