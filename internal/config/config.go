package config

import (
	"go.uber.org/zap"
	"os"
)

func InitConfig() {
	if _, err := os.Stat("./configs"); err != nil {
		if _err := os.MkdirAll("./configs", os.ModePerm); _err != nil {
			zap.L().Fatal("Unable to create directory for configurations.")
		}
	}
	InitApplicationCfg()
	InitPlatformCfg()
	InitSignatureCfg()
}
