package provider

import (
	"github.com/elabosak233/cloudsdale/internal/config"
	"go.uber.org/zap"
)

func InitContainerProvider() {
	switch config.AppCfg().Container.Provider {
	case "docker":
		InitDockerProvider()
	case "k8s":
		InitK8sProvider()
	default:
		zap.L().Fatal("Invalid container provider!")
	}
}
