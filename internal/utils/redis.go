package utils

import (
	"github.com/go-redis/redis/v8"
	"github.com/spf13/viper"
	"os"
)

var Rdb *redis.Client

func InitRedis() {
	Rdb = redis.NewClient(&redis.Options{
		Addr:     viper.GetString("Redis.Host") + ":" + viper.GetString("Redis.Port"),
		Password: viper.GetString("Redis.Password"),
		DB:       viper.GetInt("Redis.Db"),
	})
	_, err := Rdb.Ping(Rdb.Context()).Result()
	if err != nil {
		Logger.Error("Redis 连接错误 ", err)
		os.Exit(1)
	} else {
		Logger.Info("Redis 已连接")
	}
}
