package middlewares

import "github.com/gin-gonic/gin"

type AuthMiddleware interface {
	Auth() gin.HandlerFunc
	AuthInRole(role int64) gin.HandlerFunc
}
