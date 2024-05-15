package middleware

import (
	"github.com/elabosak233/cloudsdale/internal/extension/config"
	"github.com/gin-gonic/gin"
	"net/http"
	"os"
	"path/filepath"
	"strings"
)

func Frontend(urlPrefix string) gin.HandlerFunc {
	root := config.AppCfg().Gin.Paths.Frontend
	fileServer := http.FileServer(http.Dir(root))
	if !strings.HasSuffix(urlPrefix, "/") {
		urlPrefix = urlPrefix + "/"
	}
	staticServerPrefix := strings.TrimRight(urlPrefix, "/")
	return func(ctx *gin.Context) {
		if strings.HasPrefix(ctx.Request.URL.Path, "/api") || strings.HasPrefix(ctx.Request.URL.Path, "/docs") {
			ctx.Next()
		} else {
			ctx.Set("skip_logging", true)
			filePath := filepath.Join(root, ctx.Request.URL.Path)
			_, err := os.Stat(filePath)
			if err == nil {
				http.StripPrefix(staticServerPrefix, fileServer).ServeHTTP(ctx.Writer, ctx.Request)
				ctx.Abort()
			} else if os.IsNotExist(err) {
				http.ServeFile(ctx.Writer, ctx.Request, filepath.Join(root, "index.html"))
				ctx.Abort()
			} else {
				ctx.Next()
			}
		}
	}
}
