package middleware

import (
	"errors"
	"github.com/dgrijalva/jwt-go"
	"github.com/elabosak233/pgshub/model/misc"
	"github.com/gin-gonic/gin"
	"github.com/spf13/viper"
	"net/http"
)

func authMiddleware(c *gin.Context) {
	tokenString := c.GetHeader("Authorization")
	if tokenString == "" {
		c.JSON(http.StatusUnauthorized, gin.H{
			"code":  http.StatusUnauthorized,
			"error": "缺少 Token",
		})
		c.Abort()
		return
	}
	claims := &misc.Claims{}
	token, err := jwt.ParseWithClaims(tokenString, claims, func(token *jwt.Token) (interface{}, error) {
		return viper.GetString("Jwt.SecretKey"), nil
	})
	if err != nil {
		if errors.Is(err, jwt.ErrSignatureInvalid) {
			c.JSON(http.StatusUnauthorized, gin.H{
				"code":  http.StatusUnauthorized,
				"error": "无效 Token",
			})
			c.Abort()
			return
		}
		c.JSON(http.StatusUnauthorized, gin.H{"error": "Unauthorized"})
		c.Abort()
		return
	}
	if claims, ok := token.Claims.(*misc.Claims); ok && token.Valid {
		c.Set("id", claims.Id)
		c.Next()
	} else {
		c.JSON(http.StatusUnauthorized, gin.H{
			"code":  http.StatusUnauthorized,
			"error": "无效 Token",
		})
		c.Abort()
		return
	}
}
