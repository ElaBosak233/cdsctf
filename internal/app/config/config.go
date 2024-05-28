package config

import (
	"github.com/google/uuid"
	"os"
)

var (
	jwtSecretKey string
)

func JwtSecretKey() string {
	return jwtSecretKey
}

func InitConfig() {
	configPath := "configs"
	if _, err := os.Stat(configPath); os.IsNotExist(err) {
		_ = os.Mkdir(configPath, os.ModePerm)
	}

	InitApplicationCfg()
	InitPlatformCfg()
	InitSignatureCfg()

	jwtSecretKey = uuid.NewString()
}
