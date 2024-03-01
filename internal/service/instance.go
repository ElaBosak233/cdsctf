package service

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/repository"
)

type IInstanceService interface {
	FindByPodID(podIDs []uint) (containers []model.Instance, err error)
}

type InstanceService struct {
	containerRepository repository.IInstanceRepository
	envRepository       repository.IEnvRepository
	natRepository       repository.INatRepository
	imageRepository     repository.IImageRepository
	podRepository       repository.IPodRepository
	portRepository      repository.IPortRepository
}

func NewInstanceService(appRepository *repository.Repository) IInstanceService {
	return &InstanceService{
		containerRepository: appRepository.ContainerRepository,
		envRepository:       appRepository.EnvRepository,
		natRepository:       appRepository.NatRepository,
		imageRepository:     appRepository.ImageRepository,
		podRepository:       appRepository.PodRepository,
		portRepository:      appRepository.PortRepository,
	}
}

func (c *InstanceService) FindByPodID(podIDs []uint) (containers []model.Instance, err error) {
	ctns, err := c.containerRepository.FindByPodID(podIDs)
	return ctns, err
}
