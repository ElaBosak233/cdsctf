package config

import (
	"github.com/spf13/viper"
	"go.uber.org/zap"
)

var defaultSettings = map[string]interface{}{
	// 全局设置
	"global.platform.title":           "PgsHub",
	"global.platform.description":     "Hack for fun not for profit.",
	"global.container.parallel_limit": 1,    // 练习场并行容器数
	"global.container.request_limit":  30,   // 有关容器请求时间的限制（秒）
	"global.user.allow_registration":  true, // 允许新用户注册
	// 服务器设置
	"server.host":               "0.0.0.0",
	"server.port":               8888,
	"server.cors.allow_origins": []string{"*"},
	"server.cors.allow_methods": []string{"GET", "POST", "PUT", "DELETE"},
	// 邮箱设置
	"email.address":   "",
	"email.password":  "",
	"email.smtp.host": "",
	"email.smtp.port": 0,
	// 数据库设置
	"db.provider":          "postgres",
	"db.postgres.host":     "localhost",
	"db.postgres.port":     5432,
	"db.postgres.username": "pgshub",
	"db.postgres.password": "pgshub",
	"db.postgres.dbname":   "pgshub",
	"db.postgres.sslmode":  "disable",
	"db.sqlite3.filename":  "db.sqlite3",
	// JWT 设置
	"jwt.secret_key": "20101010",
	"jwt.expiration": 180,
	// 容器设置
	"container.provider":            "docker",
	"container.docker.uri":          "unix:///var/run/docker.sock", // npipe:////./pipe/docker_engine
	"container.docker.public_entry": "127.0.0.1",
	"container.docker.ports.from":   49152,
	"container.docker.ports.to":     65535,
}

func InitConfig() {
	configFile := "config.json"
	viper.SetConfigType("json")
	viper.SetConfigFile(configFile)
	if err := viper.ReadInConfig(); err != nil {
		zap.L().Warn("未找到配置文件，将创建默认配置文件")
		for key, value := range defaultSettings {
			viper.SetDefault(key, value)
		}
		if err := viper.WriteConfigAs(configFile); err != nil {
			zap.L().Error("无法创建默认配置文件")
			return
		}
		zap.L().Info("默认配置文件已生成")
	}
}

func SaveConfig() (err error) {
	configData := viper.AllSettings()
	err = viper.WriteConfig()
	if err != nil {
		return err
	}
	err = viper.MergeConfigMap(configData)
	return err
}
