package cache

import (
	goCache "github.com/patrickmn/go-cache"
	"go.uber.org/zap"
	"strings"
	"time"
)

type MemoryCache struct {
	gc *goCache.Cache
}

func NewMemoryCache() ICache {
	gc := goCache.New(5*time.Minute, 10*time.Minute)
	zap.L().Info("Cache module inits successfully. Using memory as cache provider.")
	return &MemoryCache{
		gc: gc,
	}
}
func (c *MemoryCache) Get(key string) (interface{}, bool) {
	value, exist := c.gc.Get(key)
	if exist {
		zap.L().Info("Cache hit", zap.String("key", key))
	}
	return value, exist
}

func (c *MemoryCache) Set(key string, value interface{}, expiration time.Duration) {
	zap.L().Info("Cache set", zap.String("key", key))
	c.gc.Set(key, value, expiration)
}

func (c *MemoryCache) Delete(key string) {
	c.gc.Delete(key)
}

func (c *MemoryCache) DeleteByPrefix(prefix string) {
	for k := range c.gc.Items() {
		if strings.HasPrefix(k, prefix) {
			c.gc.Delete(k)
		}
	}
}
