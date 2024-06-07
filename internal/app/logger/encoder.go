package logger

import (
	"github.com/TwiN/go-color"
	"go.uber.org/zap/zapcore"
	"time"
)

var (
	_levelToColor = map[zapcore.Level]string{
		zapcore.DebugLevel:  color.Cyan,
		zapcore.InfoLevel:   color.Cyan,
		zapcore.WarnLevel:   color.Yellow,
		zapcore.ErrorLevel:  color.Red,
		zapcore.DPanicLevel: color.Red,
		zapcore.PanicLevel:  color.Red,
		zapcore.FatalLevel:  color.Red,
	}
)

func iLevelEncoder(level zapcore.Level, enc zapcore.PrimitiveArrayEncoder) {
	c, ok := _levelToColor[level]
	if !ok {
		c = color.Cyan
	}
	enc.AppendString(color.Ize(c, " "+level.CapitalString()+" "))
}

func iTimeEncoder(t time.Time, enc zapcore.PrimitiveArrayEncoder) {
	enc.AppendString("[" + t.Format("2006-01-02 15:04:05") + "]")
}

func iCallerEncoder(caller zapcore.EntryCaller, enc zapcore.PrimitiveArrayEncoder) {
	enc.AppendString("[" + caller.TrimmedPath() + "]")
}
