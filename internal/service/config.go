package service

import (
	"github.com/elabosak233/cloudsdale/internal/config"
	"github.com/elabosak233/cloudsdale/internal/model/dto/request"
	"github.com/elabosak233/cloudsdale/internal/repository"
)

type IConfigService interface {
	FindAll() (cfg config.PlatformCfg)
	Update(req request.ConfigUpdateRequest) (err error)
}

type ConfigService struct {
}

func NewConfigService(appRepository *repository.Repository) IConfigService {
	return &ConfigService{}
}

func (c *ConfigService) FindAll() (cfg config.PlatformCfg) {
	return *(config.PltCfg())
}

func (c *ConfigService) Update(req request.ConfigUpdateRequest) (err error) {
	config.PltCfg().Site.Title = req.Platform.Title
	config.PltCfg().Site.Description = req.Platform.Description
	config.PltCfg().Container.ParallelLimit = int(req.Container.ParallelLimit)
	config.PltCfg().Container.RequestLimit = int(req.Container.RequestLimit)
	config.PltCfg().User.AllowRegistration = req.User.AllowRegistration
	err = config.PltCfg().Save()
	return err
}
