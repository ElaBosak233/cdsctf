package middleware

import (
	"github.com/gin-gonic/gin"
	"net/http"
	"strings"
)

func StaticServe(urlPrefix, root string) gin.HandlerFunc {
	fileServer := http.FileServer(http.Dir(root))
	// 如果不是以 / 结尾的，需要添加 /
	if !strings.HasSuffix(urlPrefix, "/") {
		urlPrefix = urlPrefix + "/"
	}
	// 生成静态文件服务的前缀
	staticServerPrefix := strings.TrimRight(urlPrefix, "/")
	return func(c *gin.Context) {
		if strings.HasPrefix(c.Request.URL.Path, "/api") {
			// 如果是 API 路径，就继续后续的中间件或路由处理
			c.Next()
		} else {
			// 否则就是静态文件请求，交给静态文件服务处理
			// 通过重写请求的 URL 路径，去掉前缀
			http.StripPrefix(staticServerPrefix, fileServer).ServeHTTP(c.Writer, c.Request)
			// 处理完毕后，不再调用后续的中间件或路由
			c.Abort()
		}
	}
}
