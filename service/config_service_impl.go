package service

import (
	"github.com/elabosak233/pgshub/repository"
	"github.com/spf13/viper"
)

type ConfigServiceImpl struct {
}

func NewConfigServiceImpl(appRepository *repository.AppRepository) ConfigService {
	return &ConfigServiceImpl{}
}

func (c ConfigServiceImpl) FindAll() map[string]any {
	return viper.GetStringMap("Global")
}
