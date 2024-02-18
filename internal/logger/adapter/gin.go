package adapter

import (
	"errors"
	"fmt"
	"github.com/TwiN/go-color"
	"github.com/gin-gonic/gin"
	"go.uber.org/zap"
	"net"
	"net/http/httputil"
	"os"
	"runtime/debug"
	"strings"
	"time"
)

func GinLogger() gin.HandlerFunc {
	renderStatus := func(status int) string {
		s := fmt.Sprintf(" %d ", status)
		switch {
		case status == 200:
			return color.InWhiteOverCyan(s)
		default:
			return color.InWhiteOverYellow(s)
		}
	}
	return func(c *gin.Context) {
		start := time.Now()
		path := c.Request.URL.Path
		query := c.Request.URL.RawQuery
		c.Next()
		duration := time.Since(start)
		zap.L().Info(fmt.Sprintf(
			"[%s] %s %s",
			color.InCyan("GIN"),
			renderStatus(c.Writer.Status()),
			color.InBold(path)),
			zap.Int("status", c.Writer.Status()),
			zap.String("method", c.Request.Method),
			zap.String("path", path),
			zap.String("query", query),
			zap.String("ip", c.ClientIP()),
			zap.String("user-agent", c.Request.UserAgent()),
			zap.String("errors", c.Errors.ByType(gin.ErrorTypePrivate).String()),
			zap.Duration("duration", duration),
		)
	}
}

func GinRecovery(stack bool) gin.HandlerFunc {
	return func(c *gin.Context) {
		defer func() {
			if err := recover(); err != nil {
				var brokenPipe bool
				var ne *net.OpError
				if errors.As(err.(error), &ne) {
					var se *os.SyscallError
					if errors.As(ne.Err, &se) {
						if strings.Contains(strings.ToLower(se.Error()), "broken pipe") || strings.Contains(strings.ToLower(se.Error()), "connection reset by peer") {
							brokenPipe = true
						}
					}
				}
				httpRequest, _ := httputil.DumpRequest(c.Request, false)
				if brokenPipe {
					zap.L().Error(c.Request.URL.Path,
						zap.Any("error", err),
						zap.String("request", string(httpRequest)),
					)
					_ = c.Error(err.(error))
					c.Abort()
					return
				}

				if stack {
					zap.L().Error("Recovery from panic.",
						zap.Any("error", err),
						zap.String("request", string(httpRequest)),
						zap.String("stack", string(debug.Stack())),
					)
				} else {
					zap.L().Error("Recovery from panic.",
						zap.Any("error", err),
						zap.String("request", string(httpRequest)),
					)
				}
				c.Abort()
			}
		}()
		c.Next()
	}
}
