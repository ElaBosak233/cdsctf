package services

import (
	"github.com/elabosak233/pgshub/models/entity"
	"github.com/elabosak233/pgshub/repositories"
)

type ContainerService interface {
	FindByPodID(podIDs []int64) (containers []entity.Container, err error)
}

type ContainerServiceImpl struct {
	MixinService        MixinService
	ContainerRepository repositories.ContainerRepository
	EnvRepository       repositories.EnvRepository
	NatRepository       repositories.NatRepository
	ImageRepository     repositories.ImageRepository
	PodRepository       repositories.PodRepository
	PortRepository      repositories.PortRepository
}

func NewContainerServiceImpl(appRepository *repositories.Repositories) ContainerService {
	return &ContainerServiceImpl{
		MixinService:        NewMixinServiceImpl(appRepository),
		ContainerRepository: appRepository.ContainerRepository,
		EnvRepository:       appRepository.EnvRepository,
		NatRepository:       appRepository.NatRepository,
		ImageRepository:     appRepository.ImageRepository,
		PodRepository:       appRepository.PodRepository,
		PortRepository:      appRepository.PortRepository,
	}
}

func (c *ContainerServiceImpl) FindByPodID(podIDs []int64) (containers []entity.Container, err error) {
	ctns, err := c.ContainerRepository.FindByPodID(podIDs)
	containers, err = c.MixinService.MixinContainer(ctns)
	return containers, err
}
