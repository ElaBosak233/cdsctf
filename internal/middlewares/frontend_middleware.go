package middlewares

import "github.com/gin-gonic/gin"

type FrontendMiddleware interface {
	Frontend(urlPrefix, root string) gin.HandlerFunc
}
