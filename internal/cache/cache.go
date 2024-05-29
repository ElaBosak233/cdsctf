package cache

import (
	"github.com/elabosak233/cloudsdale/internal/app/config"
	"sync"
	"time"
)

var (
	cache     ICache
	onceCache sync.Once
)

type ICache interface {
	Get(key string) (interface{}, bool)
	Set(key string, value interface{}, expiration time.Duration)
	Delete(key string)
	DeleteByPrefix(prefix string)
}

func C() ICache {
	return cache
}

func InitCache() {
	onceCache.Do(func() {
		switch config.AppCfg().Gin.Cache.Provider {
		case "memory":
			cache = NewMemoryCache()
		case "redis":
			cache = NewRedisCache()
		}
	})
}
