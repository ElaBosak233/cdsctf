package config

import (
	"github.com/elabosak233/cloudsdale/internal/utils"
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
	if _, err := os.Stat(utils.ConfigsPath); os.IsNotExist(err) {
		_ = os.Mkdir(utils.ConfigsPath, os.ModePerm)
	}
	InitApplicationCfg()
	InitPlatformCfg()
	jwtSecretKey = uuid.NewString()
}
