package services

import (
	"github.com/elabosak233/pgshub/internal/models/request"
	"github.com/elabosak233/pgshub/internal/repositories"
	"github.com/elabosak233/pgshub/internal/utils"
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

func (c ConfigServiceImpl) FindAll() (config map[string]any) {
	return viper.GetStringMap("global")
}

func (c ConfigServiceImpl) Update(req request.ConfigUpdateRequest) (err error) {
	viper.Set("global.title", req.Title)
	viper.Set("global.bio", req.Bio)
	viper.Set("global.parallel_container_limit", req.ParallelContainerLimit)
	viper.Set("global.container_request_limit", req.ContainerRequestLimit)
	return utils.SaveConfig()
}
