package service

import (
	"github.com/elabosak233/cloudsdale/internal/app/config"
	"github.com/elabosak233/cloudsdale/internal/model/request"
)

type IConfigService interface {
	Update(req request.ConfigUpdateRequest) error
}

type ConfigService struct {
}

func NewConfigService() IConfigService {
	return &ConfigService{}
}

func (c *ConfigService) Update(req request.ConfigUpdateRequest) error {
	config.PltCfg().Site.Title = req.Site.Title
	config.PltCfg().Site.Description = req.Site.Description
	config.PltCfg().Container.ParallelLimit = req.Container.ParallelLimit
	config.PltCfg().Container.RequestLimit = req.Container.RequestLimit
	config.PltCfg().User.Register.Enabled = req.User.Register.Enabled
	config.PltCfg().User.Register.Captcha.Enabled = req.User.Register.Captcha.Enabled
	config.PltCfg().User.Register.Email.Enabled = req.User.Register.Email.Enabled
	err := config.PltCfg().Save()
	return err
}
