package implements

import (
	"github.com/elabosak233/pgshub/internal/middlewares"
	"github.com/gin-gonic/gin"
	"net/http"
	"os"
	"path"
	"strings"
)

type FrontendMiddlewareImpl struct {
}

func NewFrontendMiddleware() middlewares.FrontendMiddleware {
	return &FrontendMiddlewareImpl{}
}

func (m *FrontendMiddlewareImpl) Frontend(urlPrefix, root string) gin.HandlerFunc {
	fileServer := http.FileServer(http.Dir(root))
	if !strings.HasSuffix(urlPrefix, "/") {
		urlPrefix = urlPrefix + "/" // 如果不是以 / 结尾的，需要添加 /
	}
	staticServerPrefix := strings.TrimRight(urlPrefix, "/") // 生成静态文件服务的前缀
	return func(ctx *gin.Context) {
		if strings.HasPrefix(ctx.Request.URL.Path, "/api") || strings.HasPrefix(ctx.Request.URL.Path, "/docs") {
			ctx.Next()
		} else {
			filePath := path.Join(root, ctx.Request.URL.Path)
			_, err := os.Stat(filePath)
			if err != nil {
				http.ServeFile(ctx.Writer, ctx.Request, path.Join(root, "404.html"))
				ctx.Abort()
			} else {
				http.StripPrefix(staticServerPrefix, fileServer).ServeHTTP(ctx.Writer, ctx.Request)
				ctx.Abort()
			}
		}
	}
}
