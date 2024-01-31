package logger

import (
	"fmt"
	"github.com/gin-gonic/gin"
	"github.com/mattn/go-colorable"
	"github.com/sirupsen/logrus"
	"gopkg.in/natefinch/lumberjack.v2"
	"io"
	"time"
)

var log *logrus.Logger

func init() {
	filename := fmt.Sprintf("logs/%d.log", time.Now().Unix())
	logWriter := &lumberjack.Logger{
		Filename:   filename,
		MaxSize:    1,
		MaxBackups: 3,
		MaxAge:     7,
		Compress:   true,
	}
	log = logrus.New()
	log.SetLevel(logrus.TraceLevel)
	log.SetFormatter(&IFormatter{})
	log.SetOutput(io.MultiWriter(colorable.NewColorableStdout(), logWriter))
	gin.DefaultWriter = log.Writer()
	log.Info("日志系统已启动")
}

func Get() *logrus.Logger {
	return log
}

func Info(args ...any) {
	log.Info(args...)
}

func Debug(args ...any) {
	log.Debug(args...)
}

func Warn(args ...any) {
	log.Warn(args...)
}

func Error(args ...any) {
	log.Error(args...)
}

func WithFields(fields logrus.Fields) *logrus.Entry {
	return log.WithFields(fields)
}

func WithField(key string, value interface{}) *logrus.Entry {
	return log.WithField(key, value)
}

func Fatal(args ...any) {
	log.Fatal(args...)
}

func Debugf(format string, args ...interface{}) {
	log.Debugf(format, args...)
}

func Infof(format string, args ...interface{}) {
	log.Infof(format, args...)
}

func Warnf(format string, args ...interface{}) {
	log.Warnf(format, args...)
}

func Errorf(format string, args ...interface{}) {
	log.Errorf(format, args...)
}

func Debugln(args ...interface{}) {
	log.Debugln(args...)
}
