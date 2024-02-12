package services

import (
	"github.com/elabosak233/pgshub/models/request"
	"github.com/elabosak233/pgshub/repositories"
	"github.com/elabosak233/pgshub/utils/config"
)

type ConfigService interface {
	FindAll() (cfg config.Global)
	Update(req request.ConfigUpdateRequest) (err error)
}

type ConfigServiceImpl struct {
}

func NewConfigServiceImpl(appRepository *repositories.Repositories) ConfigService {
	return &ConfigServiceImpl{}
}

func (c *ConfigServiceImpl) FindAll() (cfg config.Global) {
	return config.Cfg().Global
}

func (c *ConfigServiceImpl) Update(req request.ConfigUpdateRequest) (err error) {
	config.Cfg().Global.Platform.Title = req.Platform.Title
	config.Cfg().Global.Platform.Description = req.Platform.Description
	config.Cfg().Global.Container.ParallelLimit = int(req.Container.ParallelLimit)
	config.Cfg().Global.Container.RequestLimit = int(req.Container.RequestLimit)
	config.Cfg().Global.User.AllowRegistration = req.User.AllowRegistration
	err = config.SaveConfig()
	return err
}
