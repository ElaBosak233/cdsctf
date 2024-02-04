package logger

import (
	"github.com/TwiN/go-color"
	"go.uber.org/zap/zapcore"
	"time"
)

var (
	_levelToColor = map[zapcore.Level]string{
		zapcore.DebugLevel:  color.CyanBackground,
		zapcore.InfoLevel:   color.CyanBackground,
		zapcore.WarnLevel:   color.YellowBackground,
		zapcore.ErrorLevel:  color.RedBackground,
		zapcore.DPanicLevel: color.RedBackground,
		zapcore.PanicLevel:  color.RedBackground,
		zapcore.FatalLevel:  color.RedBackground,
	}
)

func iLevelEncoder(level zapcore.Level, enc zapcore.PrimitiveArrayEncoder) {
	c, ok := _levelToColor[level]
	if !ok {
		c = color.CyanBackground
	}
	enc.AppendString(color.Ize(color.White+c, " "+level.CapitalString()+" "))
}

func iTimeEncoder(t time.Time, enc zapcore.PrimitiveArrayEncoder) {
	enc.AppendString("[" + t.Format("2006-01-02 15:04:05") + "]")
}

func iCallerEncoder(caller zapcore.EntryCaller, enc zapcore.PrimitiveArrayEncoder) {
	enc.AppendString("[" + caller.TrimmedPath() + "]")
}
