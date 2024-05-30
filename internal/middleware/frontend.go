package middleware

import (
	"github.com/elabosak233/cloudsdale/internal/app/config"
	"github.com/elabosak233/cloudsdale/internal/utils"
	"github.com/gin-gonic/gin"
	"net/http"
	"os"
	"path/filepath"
	"strings"
)

func index(ctx *gin.Context) {
	filePath := filepath.Join(utils.FrontendPath, "index.html")
	indexContent, err := os.ReadFile(filePath)
	if err != nil {
		ctx.JSON(http.StatusInternalServerError, gin.H{
			"code": http.StatusInternalServerError,
			"msg":  "Error reading index.html",
		})
		ctx.Abort()
		return
	}
	indexContentStr := string(indexContent)
	indexContentStr = strings.ReplaceAll(indexContentStr, "{{ Cloudsdale.Title }}", config.PltCfg().Site.Title)
	ctx.Header("Content-Type", "text/html; charset=utf-8")
	ctx.String(http.StatusOK, indexContentStr)
	ctx.Abort()
}

func Frontend(urlPrefix string) gin.HandlerFunc {
	fileServer := http.FileServer(http.Dir(utils.FrontendPath))
	if !strings.HasSuffix(urlPrefix, "/") {
		urlPrefix = urlPrefix + "/"
	}
	staticServerPrefix := strings.TrimRight(urlPrefix, "/")
	return func(ctx *gin.Context) {
		if strings.HasPrefix(ctx.Request.URL.Path, "/api") || strings.HasPrefix(ctx.Request.URL.Path, "/docs") {
			ctx.Next()
		} else {
			ctx.Set("skip_logging", true)
			filePath := filepath.Join(utils.FrontendPath, ctx.Request.URL.Path)
			_, err := os.Stat(filePath)
			if err == nil {
				if ctx.Request.URL.Path == "/" || ctx.Request.URL.Path == "/index.html" {
					index(ctx)
				} else {
					http.StripPrefix(staticServerPrefix, fileServer).ServeHTTP(ctx.Writer, ctx.Request)
					ctx.Abort()
				}
			} else if os.IsNotExist(err) {
				index(ctx)
			} else {
				ctx.Next()
			}
		}
	}
}
