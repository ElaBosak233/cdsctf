package utils

import (
	"github.com/spf13/viper"
)

func LoadConfig() {
	configFile := "config.json"

	// 使用Viper读取配置文件
	viper.SetConfigFile(configFile)

	// 读取配置文件，如果文件不存在则创建并写入默认配置
	if err := viper.ReadInConfig(); err != nil {
		Logger.Warn("未找到配置文件，将创建默认配置文件")
		defaults := map[string]interface{}{
			"Server.Host":          "0.0.0.0",
			"Server.Port":          8888,
			"MySql.Host":           "localhost",
			"MySql.Port":           3306,
			"MySql.Username":       "pgshub",
			"MySql.Password":       "pgshub",
			"MySql.DbName":         "pgshub",
			"Jwt.SecretKey":        "20101010",
			"Jwt.ExpirationTime":   180,
			"Container.Host":       "0.0.0.0",
			"Container.Ports.From": 49152,
			"Container.Ports.To":   65535,
			"Redis.Host":           "localhost",
			"Redis.Port":           6379,
		}

		for key, value := range defaults {
			viper.SetDefault(key, value)
		}

		// 保存默认配置到文件
		if err := viper.WriteConfigAs(configFile); err != nil {
			Logger.Error("无法创建默认配置文件")
			return
		}
		Logger.Info("默认配置文件已生成")
	}
}
