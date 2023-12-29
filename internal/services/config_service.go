package services

import (
	"github.com/elabosak233/pgshub/internal/repositories"
	"github.com/spf13/viper"
)

type ConfigService interface {
	FindAll() map[string]any
}

type ConfigServiceImpl struct {
}

func NewConfigServiceImpl(appRepository *repositories.AppRepository) ConfigService {
	return &ConfigServiceImpl{}
}

func (c ConfigServiceImpl) FindAll() map[string]any {
	return viper.GetStringMap("Global")
}
