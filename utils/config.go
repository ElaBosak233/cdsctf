package utils

import (
	"github.com/spf13/viper"
)

func LoadConfig() {
	configFile := "config.json"
	viper.SetConfigFile(configFile)
	if err := viper.ReadInConfig(); err != nil {
		Logger.Warn("未找到配置文件，将创建默认配置文件")
		defaults := map[string]interface{}{
			"Title":              "PgsHub",
			"Server.Host":        "0.0.0.0",
			"Server.Port":        8888,
			"MySql.Host":         "localhost",
			"MySql.Port":         3306,
			"MySql.Username":     "pgshub",
			"MySql.Password":     "pgshub",
			"MySql.DbName":       "pgshub",
			"Jwt.SecretKey":      "20101010",
			"Jwt.ExpirationTime": 180,
			"Container":          "docker",
			"Docker.Entry":       "0.0.0.0",
			"Docker.Ports.From":  49152,
			"Docker.Ports.To":    65535,
			"Redis.Host":         "localhost",
			"Redis.Port":         6379,
			"Redis.Password":     "",
			"Redis.Db":           0,
		}
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
