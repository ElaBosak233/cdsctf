package cache

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"github.com/elabosak233/cloudsdale/internal/app/config"
	"github.com/gin-gonic/gin"
	"github.com/redis/go-redis/v9"
	"go.uber.org/zap"
	"time"
)

type RedisCache struct {
	ctx context.Context
	rdb *redis.Client
}

func NewRedisCache() ICache {
	rdb := redis.NewClient(&redis.Options{
		Addr:     fmt.Sprintf("%s:%d", config.AppCfg().Gin.Cache.Redis.Host, config.AppCfg().Gin.Cache.Redis.Port),
		Password: config.AppCfg().Gin.Cache.Redis.Password,
		DB:       config.AppCfg().Gin.Cache.Redis.DB,
	})
	zap.L().Info("Cache module inits successfully. Using Redis as cache provider.")
	return &RedisCache{
		ctx: context.Background(),
		rdb: rdb,
	}
}

func (r *RedisCache) Set(key string, value interface{}, expiration time.Duration) {
	h, ok := value.(gin.H)
	if !ok {
		zap.L().Error("Value is not of type gin.H", zap.Any("value", value))
		return
	}

	jsonData, err := json.Marshal(h)
	if err != nil {
		zap.L().Error("Error marshalling gin.H to JSON", zap.Error(err))
		return
	}

	if err := r.rdb.Set(r.ctx, key, jsonData, expiration).Err(); err != nil {
		zap.L().Error("Error setting cache", zap.String("key", key), zap.Error(err))
	}
	zap.L().Info("Cache set", zap.String("key", key))
}

func (r *RedisCache) Get(key string) (interface{}, bool) {
	jsonData, err := r.rdb.Get(r.ctx, key).Result()
	if errors.Is(err, redis.Nil) {
		zap.L().Info("Cache miss", zap.String("key", key))
		return nil, false
	} else if err != nil {
		zap.L().Error("Error retrieving from cache", zap.String("key", key), zap.Error(err))
		return nil, false
	}

	var h gin.H
	if err := json.Unmarshal([]byte(jsonData), &h); err != nil {
		zap.L().Error("Error unmarshalling JSON to gin.H", zap.Error(err))
		return nil, false
	}

	zap.L().Info("Cache hit", zap.String("key", key))
	return h, true
}

func (r *RedisCache) Delete(key string) {
	if err := r.rdb.Del(r.ctx, key).Err(); err != nil {
		zap.L().Error("Error deleting from cache", zap.String("key", key), zap.Error(err))
	}
}

func (r *RedisCache) DeleteByPrefix(prefix string) {
	var cursor uint64
	var keys []string
	var err error
	for {
		keys, cursor, err = r.rdb.Scan(r.ctx, cursor, fmt.Sprintf("%s*", prefix), 10).Result()
		if err != nil {
			zap.L().Error("Error scanning cache for prefix", zap.String("prefix", prefix), zap.Error(err))
			break
		}
		if len(keys) > 0 {
			if err := r.rdb.Del(r.ctx, keys...).Err(); err != nil {
				zap.L().Error("Error deleting keys by prefix", zap.String("prefix", prefix), zap.Error(err))
			}
		}
		if cursor == 0 {
			break
		}
	}
}
