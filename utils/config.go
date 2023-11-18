package utils

import (
	"fmt"
	model "github.com/elabosak233/pgshub/model/config"
	"gopkg.in/yaml.v3"
	"os"
)

func LoadConfig() (*model.Config, error) {
	configFile := "config.yml"
	if _, err := os.Stat(configFile); os.IsNotExist(err) {
		defaultConfig := &model.Config{
			Server: model.ServerConfig{
				Port: 8888,
			},
			Database: model.DatabaseConfig{
				Host:     "localhost",
				Port:     3306,
				Username: "root",
				Password: "password",
			},
		}
		defaultConfigYAML, err := yaml.Marshal(defaultConfig)
		ErrorPanic(err)
		err = os.WriteFile(configFile, defaultConfigYAML, 0644)
		ErrorPanic(err)
		fmt.Println("Default configuration file created.")
	}
	fileContent, err := os.ReadFile(configFile)
	ErrorPanic(err)
	var cfg model.Config
	err = yaml.Unmarshal(fileContent, &cfg)
	ErrorPanic(err)
	return &cfg, nil
}
