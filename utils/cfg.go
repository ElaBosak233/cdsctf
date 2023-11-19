package utils

import (
	model "github.com/elabosak233/pgshub/model/config"
	"gopkg.in/yaml.v3"
	"os"
)

var Cfg model.Config

func LoadConfig() {
	configFile := "config.yml"
	if _, err := os.Stat(configFile); os.IsNotExist(err) {
		Logger.Warn("未找到配置文件，将创建默认配置文件")
		defaultConfig := &model.Config{
			Server: model.ServerConfig{
				Host: "0.0.0.0",
				Port: 8888,
			},
			Database: model.DatabaseConfig{
				Host:     "localhost",
				Port:     3306,
				Username: "root",
				Password: "password",
			},
			Jwt: model.JwtConfig{
				SecretKey:      "20101010",
				ExpirationTime: 180,
			},
			Container: model.ContainerConfig{
				Host: "0.0.0.0",
				Ports: model.Ports{
					From: 49152,
					To:   65535,
				},
			},
		}
		defaultConfigYAML, _ := yaml.Marshal(defaultConfig)
		err = os.WriteFile(configFile, defaultConfigYAML, 0644)
		if err != nil {
			Logger.Error("无法创建默认配置文件")
			return
		}
		Logger.Info("默认配置文件已生成")
	}
	fileContent, err := os.ReadFile(configFile)
	if err != nil {
		Logger.Error("无法读取配置文件")
		return
	}
	err = yaml.Unmarshal(fileContent, &Cfg)
	if err != nil {
		Logger.Error("无法解析配置文件")
	}
}
