package logger

import (
	"fmt"
	"github.com/TwiN/go-color"
	"go.uber.org/zap"
	"go.uber.org/zap/zapcore"
	"xorm.io/xorm/log"
)

type Adapter struct {
	logger  *zap.Logger
	level   log.LogLevel
	showSQL bool
}

func Logger(logger *zap.Logger) log.ContextLogger {
	return &Adapter{
		logger: logger,
	}
}

func (l *Adapter) BeforeSQL(_ log.LogContext) {}

func (l *Adapter) AfterSQL(lc log.LogContext) {
	if !l.showSQL {
		return
	}
	lg := l.logger
	sql := fmt.Sprintf("%v %v", lc.SQL, lc.Args)
	var level zapcore.Level
	if lc.Err != nil {
		level = zapcore.ErrorLevel
	} else {
		level = zapcore.InfoLevel
	}

	lg.Check(level, fmt.Sprintf("[%s] %s", color.InYellow("SQL"), sql)).Write(zap.Duration("duration", lc.ExecuteTime), zap.Error(lc.Err))
}

func (l *Adapter) Debugf(format string, v ...interface{}) {
	l.logger.Sugar().Debugf(format, v...)
}

func (l *Adapter) Errorf(format string, v ...interface{}) {
	l.logger.Sugar().Errorf(format, v...)
}

func (l *Adapter) Infof(format string, v ...interface{}) {
	l.logger.Sugar().Infof(format, v...)
}

func (l *Adapter) Warnf(format string, v ...interface{}) {
	l.logger.Sugar().Warnf(format, v...)
}

func (l *Adapter) Level() log.LogLevel {
	return l.level
}

func (l *Adapter) SetLevel(lv log.LogLevel) {
	l.level = lv
}

func (l *Adapter) ShowSQL(show ...bool) {
	if len(show) == 0 {
		l.showSQL = true
		return
	}
	l.showSQL = show[0]
}

func (l *Adapter) IsShowSQL() bool {
	return l.showSQL
}
