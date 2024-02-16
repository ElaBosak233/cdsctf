package logger

import (
	"github.com/gin-gonic/gin"
	"go.uber.org/zap"
	"go.uber.org/zap/zapcore"
	"gopkg.in/natefinch/lumberjack.v2"
	"os"
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
