package service

import (
	"github.com/elabosak233/pgshub/internal/model"
	"github.com/elabosak233/pgshub/internal/repository"
)

type IInstanceService interface {
	FindByPodID(podIDs []uint) (containers []model.Instance, err error)
}

type Instance struct {
	ContainerRepository repository.IInstanceRepository
	EnvRepository       repository.IEnvRepository
	NatRepository       repository.INatRepository
	ImageRepository     repository.IImageRepository
	PodRepository       repository.IPodRepository
	PortRepository      repository.IPortRepository
}

func NewInstanceService(appRepository *repository.Repository) IInstanceService {
	return &Instance{
		ContainerRepository: appRepository.ContainerRepository,
		EnvRepository:       appRepository.EnvRepository,
		NatRepository:       appRepository.NatRepository,
		ImageRepository:     appRepository.ImageRepository,
		PodRepository:       appRepository.PodRepository,
		PortRepository:      appRepository.PortRepository,
	}
}

func (c *Instance) FindByPodID(podIDs []uint) (containers []model.Instance, err error) {
	ctns, err := c.ContainerRepository.FindByPodID(podIDs)
	return ctns, err
}
