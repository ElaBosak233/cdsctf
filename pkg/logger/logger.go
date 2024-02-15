package logger

import (
	"errors"
	"fmt"
	"github.com/TwiN/go-color"
	"github.com/gin-gonic/gin"
	"go.uber.org/zap"
	"go.uber.org/zap/zapcore"
	"gopkg.in/natefinch/lumberjack.v2"
	"net"
	"net/http/httputil"
	"os"
	"runtime/debug"
	"strings"
	"time"
)

var (
	lg      *zap.Logger
	sugarLg *zap.SugaredLogger
)

type Cfg struct {
	Level      string `json:"level"`
	Filename   string `json:"filename"`
	MaxSize    int    `json:"maxsize"`
	MaxAge     int    `json:"max_age"`
	MaxBackups int    `json:"max_backups"`
}

var (
	cfg = Cfg{
		Level:      "debug",
		Filename:   "logs/log.log",
		MaxSize:    100,
		MaxAge:     30,
		MaxBackups: 30,
	}
)

func InitLogger() {
	writeSyncer := getLogWriter(cfg.Filename, cfg.MaxSize, cfg.MaxBackups, cfg.MaxAge)
	var l = new(zapcore.Level)
	err := l.UnmarshalText([]byte(cfg.Level))
	// Create a console encoder config with color
	consoleEncoderConfig := zapcore.EncoderConfig{
		LevelKey:         "level",
		TimeKey:          "ts",
		CallerKey:        "",
		MessageKey:       "msg",
		NameKey:          "logger",
		StacktraceKey:    "stacktrace",
		EncodeLevel:      iLevelEncoder,
		EncodeTime:       iTimeEncoder,
		EncodeCaller:     iCallerEncoder,
		EncodeDuration:   zapcore.StringDurationEncoder,
		ConsoleSeparator: " ",
	}
	consoleEncoder := zapcore.NewConsoleEncoder(consoleEncoderConfig)
	consoleCore := zapcore.NewCore(consoleEncoder, zapcore.AddSync(os.Stdout), l)
	gin.DefaultWriter = zapcore.AddSync(os.Stdout)
	encoder := getEncoder()
	core := zapcore.NewCore(encoder, writeSyncer, l)
	multiCore := zapcore.NewTee(core, consoleCore)
	lg = zap.New(multiCore, zap.AddCaller())
	sugarLg = lg.Sugar()
	zap.ReplaceGlobals(lg)
	if err != nil {
		panic(err)
	}
	zap.L().Info("The logging system has started.")
}

func L() *zap.Logger {
	return lg
}

func getEncoder() zapcore.Encoder {
	encoderConfig := zap.NewProductionEncoderConfig()
	encoderConfig.EncodeTime = zapcore.ISO8601TimeEncoder
	encoderConfig.TimeKey = "time"
	encoderConfig.EncodeLevel = zapcore.CapitalLevelEncoder
	encoderConfig.EncodeDuration = zapcore.SecondsDurationEncoder
	encoderConfig.EncodeCaller = zapcore.ShortCallerEncoder
	return zapcore.NewJSONEncoder(encoderConfig)
}

func getLogWriter(filename string, maxSize, maxBackup, maxAge int) zapcore.WriteSyncer {
	lumberJackLogger := &lumberjack.Logger{
		Filename:   filename,
		MaxSize:    maxSize,
		MaxBackups: maxBackup,
		MaxAge:     maxAge,
	}
	return zapcore.AddSync(lumberJackLogger)
}

func GinLogger() gin.HandlerFunc {
	return func(c *gin.Context) {
		start := time.Now()
		path := c.Request.URL.Path
		query := c.Request.URL.RawQuery
		c.Next()
		duration := time.Since(start)
		lg.Info(fmt.Sprintf(
			"[%s] %s",
			color.InCyan("GIN"),
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
					lg.Error(c.Request.URL.Path,
						zap.Any("error", err),
						zap.String("request", string(httpRequest)),
					)
					_ = c.Error(err.(error))
					c.Abort()
					return
				}

				if stack {
					lg.Error("Recovery from panic.",
						zap.Any("error", err),
						zap.String("request", string(httpRequest)),
						zap.String("stack", string(debug.Stack())),
					)
				} else {
					lg.Error("Recovery from panic.",
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
