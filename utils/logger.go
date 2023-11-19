package utils

import (
	"fmt"
	"github.com/gin-gonic/gin"
	"github.com/mattn/go-colorable"
	"github.com/sirupsen/logrus"
	"gopkg.in/natefinch/lumberjack.v2"
	"io"
	"time"
)

var Logger *logrus.Logger

func InitLogger() {
	logWriter := &lumberjack.Logger{
		Filename:   fmt.Sprintf("logs/%d.log", time.Now().Unix()), // 日志文件的名称
		MaxSize:    50,                                            // 每个日志文件的最大大小，单位是 MB
		MaxBackups: 3,                                             // 保留的旧日志文件的最大数量
		MaxAge:     28,                                            // 保留的旧日志文件的最大天数
		Compress:   true,                                          // 是否压缩旧日志文件
	}
	Logger = logrus.New()
	Logger.SetLevel(logrus.TraceLevel)
	Logger.SetFormatter(&logrus.TextFormatter{
		ForceColors:     true,
		TimestampFormat: "2006-01-02 15:04:05",
		FullTimestamp:   true,
	})
	Logger.SetOutput(io.MultiWriter(colorable.NewColorableStdout(), logWriter))
	gin.DefaultWriter = Logger.Writer()
	Logger.Info("日志系统已启动")

}
