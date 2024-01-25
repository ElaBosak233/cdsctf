package services

import (
	"github.com/elabosak233/pgshub/models/request"
	"github.com/elabosak233/pgshub/repositories"
	"github.com/elabosak233/pgshub/utils"
	"github.com/spf13/viper"
)

type ConfigService interface {
	FindAll() (config map[string]any)
	Update(req request.ConfigUpdateRequest) (err error)
}

type ConfigServiceImpl struct {
}

func NewConfigServiceImpl(appRepository *repositories.AppRepository) ConfigService {
	return &ConfigServiceImpl{}
}

func (c *ConfigServiceImpl) FindAll() (config map[string]any) {
	return viper.GetStringMap("global")
}

func (c *ConfigServiceImpl) Update(req request.ConfigUpdateRequest) (err error) {
	viper.Set("global.platform.title", req.Platform.Title)
	viper.Set("global.platform.description", req.Platform.Description)
	viper.Set("global.container.parallel_limit", req.Container.ParallelLimit)
	viper.Set("global.container.request_limit", req.Container.RequestLimit)
	viper.Set("global.user.allow_registration", req.User.AllowRegistration)
	return utils.SaveConfig()
}
