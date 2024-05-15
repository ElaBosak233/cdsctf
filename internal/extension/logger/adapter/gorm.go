package adapter

import (
	"context"
	"errors"
	"fmt"
	"github.com/TwiN/go-color"
	"go.uber.org/zap"
	"go.uber.org/zap/zapcore"
	"gorm.io/gorm"
	gormLogger "gorm.io/gorm/logger"
	"path/filepath"
	"runtime"
	"strings"
	"time"
)

var (
	gormPackage = filepath.Join("gorm.io", "gorm")
)

type ContextFn func(ctx context.Context) []zapcore.Field

// GORMAdapter
// Modified from "moul.io/zapgorm2"
type GORMAdapter struct {
	ZapLogger                 *zap.Logger
	LogLevel                  gormLogger.LogLevel
	SlowThreshold             time.Duration
	SkipCallerLookup          bool
	IgnoreRecordNotFoundError bool
	Context                   ContextFn
}

func NewGORMAdapter(zapLogger *zap.Logger) GORMAdapter {
	return GORMAdapter{
		ZapLogger:                 zapLogger,
		LogLevel:                  gormLogger.Warn,
		SlowThreshold:             100 * time.Millisecond,
		SkipCallerLookup:          false,
		IgnoreRecordNotFoundError: false,
		Context:                   nil,
	}
}

func (a GORMAdapter) SetAsDefault() {
	gormLogger.Default = a
}

func (a GORMAdapter) LogMode(level gormLogger.LogLevel) gormLogger.Interface {
	return GORMAdapter{
		ZapLogger:                 a.ZapLogger,
		SlowThreshold:             a.SlowThreshold,
		LogLevel:                  level,
		SkipCallerLookup:          a.SkipCallerLookup,
		IgnoreRecordNotFoundError: a.IgnoreRecordNotFoundError,
		Context:                   a.Context,
	}
}

func (a GORMAdapter) Info(ctx context.Context, str string, args ...interface{}) {
	if a.LogLevel < gormLogger.Info {
		return
	}
	a.logger(ctx).Sugar().Debugf(str, args...)
}

func (a GORMAdapter) Warn(ctx context.Context, str string, args ...interface{}) {
	if a.LogLevel < gormLogger.Warn {
		return
	}
	a.logger(ctx).Sugar().Warnf(str, args...)
}

func (a GORMAdapter) Error(ctx context.Context, str string, args ...interface{}) {
	if a.LogLevel < gormLogger.Error {
		return
	}
	a.logger(ctx).Sugar().Errorf(str, args...)
}

func (a GORMAdapter) Trace(ctx context.Context, begin time.Time, fc func() (string, int64), err error) {
	if a.LogLevel <= 0 {
		return
	}
	sql, rows := fc()
	trace := fmt.Sprintf(
		"[%s] %s",
		color.InCyan("GORM"),
		color.InBold(sql),
	)
	elapsed := time.Since(begin)
	logger := a.logger(ctx)
	switch {
	case err != nil && a.LogLevel >= gormLogger.Error && (!a.IgnoreRecordNotFoundError || !errors.Is(err, gorm.ErrRecordNotFound)):
		logger.Error(trace,
			zap.Error(err),
			zap.Duration("elapsed", elapsed),
			zap.Int64("rows", rows),
			zap.String("sql", sql),
		)
	case a.SlowThreshold != 0 && elapsed > a.SlowThreshold && a.LogLevel >= gormLogger.Warn:
		logger.Warn(trace,
			zap.Duration("elapsed", elapsed),
			zap.Int64("rows", rows),
			zap.String("sql", sql),
		)
	case a.LogLevel >= gormLogger.Info:
		logger.Debug(trace,
			zap.Duration("elapsed", elapsed),
			zap.Int64("rows", rows),
			zap.String("sql", sql),
		)
	}
}

func (a GORMAdapter) logger(ctx context.Context) *zap.Logger {
	logger := a.ZapLogger
	if a.Context != nil {
		fields := a.Context(ctx)
		logger = logger.With(fields...)
	}

	if a.SkipCallerLookup {
		return logger
	}

	for i := 2; i < 15; i++ {
		_, file, _, ok := runtime.Caller(i)
		switch {
		case !ok:
		case strings.HasSuffix(file, "_test.go"):
		case strings.Contains(file, gormPackage):
		default:
			return logger.WithOptions(zap.AddCallerSkip(i))
		}
	}
	return logger
}
