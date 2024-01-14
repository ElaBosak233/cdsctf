package utils

import (
	"github.com/spf13/viper"
)

var defaultSettings = map[string]interface{}{
	// 全局设置
	"global.title": "PgsHub",
	"global.bio":   "Hack for fun not for profit.",
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
	"db.provider":          "sqlite",
	"db.mysql.host":        "localhost",
	"db.mysql.port":        3306,
	"db.mysql.username":    "pgshub",
	"db.mysql.password":    "pgshub",
	"db.mysql.dbname":      "pgshub",
	"db.postgres.host":     "localhost",
	"db.postgres.port":     5432,
	"db.postgres.username": "pgshub",
	"db.postgres.password": "pgshub",
	"db.postgres.dbname":   "pgshub",
	"db.postgres.sslmode":  "disable",
	"db.sqlite.filename":   "db.sqlite3",
	// JWT 设置
	"jwt.secret_key": "20101010",
	"jwt.expiration": 180,
	// 容器设置
	"container.provider":            "docker",
	"container.docker.host":         "unix:///var/run/docker.sock", // npipe:////./pipe/docker_engine
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
		for key, value := range defaultSettings {
			viper.SetDefault(key, value)
		}
		if err := viper.WriteConfigAs(configFile); err != nil {
			Logger.Error("无法创建默认配置文件")
			return
		}
		Logger.Info("默认配置文件已生成")
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
