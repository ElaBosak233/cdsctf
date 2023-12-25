package utils

import (
	"github.com/spf13/viper"
)

var defaultSettings = map[string]interface{}{
	// 全局设置
	"global.title": "PgsHub",
	// 服务器设置
	"server.host":               "0.0.0.0",
	"server.port":               8888,
	"server.cors.allow_origins": []string{"*"},
	"server.cors.allow_methods": []string{"GET", "POST", "PUT", "DELETE"},
	// 邮箱设置
	"email.address":   "pgshub@163.com",
	"email.password":  "123456",
	"email.smtp.host": "smtp.163.com",
	"email.smtp.port": 25,
	// 数据库设置
	"db.mysql.host":     "localhost",
	"db.mysql.port":     3306,
	"db.mysql.username": "pgshub",
	"db.mysql.password": "pgshub",
	"db.mysql.dbname":   "pgshub",
	// JWT 设置
	"jwt.secret_key": "20101010",
	"jwt.expiration": 180,
	// 容器设置
	"container.provider":            "docker",
	"container.docker.public_entry": "127.0.0.1",
	"container.docker.ports.from":   49152,
	"container.docker.ports.to":     65535,
}

func LoadConfig() {
	configFile := "config.json"
	viper.SetConfigType("json")
	viper.SetConfigFile(configFile)
	if err := viper.ReadInConfig(); err != nil {
		Logger.Warn("未找到配置文件，将创建默认配置文件")
		defaults := defaultSettings
		for key, value := range defaults {
			viper.SetDefault(key, value)
		}
		if err := viper.WriteConfigAs(configFile); err != nil {
			Logger.Error("无法创建默认配置文件")
			return
		}
		Logger.Info("默认配置文件已生成")
	}
}
