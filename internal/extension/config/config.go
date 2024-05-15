package config

import "github.com/google/uuid"

var (
	jwtSecretKey string
)

func JwtSecretKey() string {
	return jwtSecretKey
}

func InitConfig() {
	InitApplicationCfg()
	InitPlatformCfg()
	InitSignatureCfg()

	jwtSecretKey = uuid.NewString()
}
