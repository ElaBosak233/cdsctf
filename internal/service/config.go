package service

import (
	config2 "github.com/elabosak233/pgshub/internal/config"
	"github.com/elabosak233/pgshub/internal/model/dto/request"
	"github.com/elabosak233/pgshub/internal/repository"
)

type IConfigService interface {
	FindAll() (cfg config2.Global)
	Update(req request.ConfigUpdateRequest) (err error)
}

type ConfigService struct {
}

func NewConfigService(appRepository *repository.Repository) IConfigService {
	return &ConfigService{}
}

func (c *ConfigService) FindAll() (cfg config2.Global) {
	return config2.Cfg().Global
}

func (c *ConfigService) Update(req request.ConfigUpdateRequest) (err error) {
	config2.Cfg().Global.Platform.Title = req.Platform.Title
	config2.Cfg().Global.Platform.Description = req.Platform.Description
	config2.Cfg().Global.Container.ParallelLimit = int(req.Container.ParallelLimit)
	config2.Cfg().Global.Container.RequestLimit = int(req.Container.RequestLimit)
	config2.Cfg().Global.User.AllowRegistration = req.User.AllowRegistration
	err = config2.SaveConfig()
	return err
}
