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

func (a *Adapter) BeforeSQL(_ log.LogContext) {}

func (a *Adapter) AfterSQL(lc log.LogContext) {
	if !a.showSQL {
		return
	}
	lg := a.logger
	sql := fmt.Sprintf("%v %v", lc.SQL, lc.Args)
	var level zapcore.Level
	if lc.Err != nil {
		level = zapcore.ErrorLevel
	} else {
		level = zapcore.InfoLevel
	}

	lg.Check(level, fmt.Sprintf("[%s] %s", color.InYellow("SQL"), sql)).Write(zap.Duration("duration", lc.ExecuteTime), zap.Error(lc.Err))
}

func (a *Adapter) Debugf(format string, v ...interface{}) {
	a.logger.Sugar().Debugf(format, v...)
}

func (a *Adapter) Errorf(format string, v ...interface{}) {
	a.logger.Sugar().Errorf(format, v...)
}

func (a *Adapter) Infof(format string, v ...interface{}) {
	a.logger.Sugar().Infof(format, v...)
}

func (a *Adapter) Warnf(format string, v ...interface{}) {
	a.logger.Sugar().Warnf(format, v...)
}

func (a *Adapter) Level() log.LogLevel {
	return a.level
}

func (a *Adapter) SetLevel(lv log.LogLevel) {
	a.level = lv
}

func (a *Adapter) ShowSQL(show ...bool) {
	if len(show) == 0 {
		a.showSQL = true
		return
	}
	a.showSQL = show[0]
}

func (a *Adapter) IsShowSQL() bool {
	return a.showSQL
}
