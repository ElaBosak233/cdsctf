package services

import (
	"github.com/elabosak233/pgshub/models/entity"
	"github.com/elabosak233/pgshub/repositories"
)

type ContainerService interface {
	FindByPodID(podIDs []int64) (containers []entity.Container, err error)
}

type ContainerServiceImpl struct {
	ImageService        ImageService
	ContainerRepository repositories.ContainerRepository
	EnvRepository       repositories.EnvRepository
	NatRepository       repositories.NatRepository
	ImageRepository     repositories.ImageRepository
	PodRepository       repositories.PodRepository
	PortRepository      repositories.PortRepository
}

func NewContainerServiceImpl(appRepository *repositories.Repositories) ContainerService {
	return &ContainerServiceImpl{
		ImageService:        NewImageServiceImpl(appRepository),
		ContainerRepository: appRepository.ContainerRepository,
		EnvRepository:       appRepository.EnvRepository,
		NatRepository:       appRepository.NatRepository,
		ImageRepository:     appRepository.ImageRepository,
		PodRepository:       appRepository.PodRepository,
		PortRepository:      appRepository.PortRepository,
	}
}

func (c *ContainerServiceImpl) Mixin(containers []entity.Container) (ctns []entity.Container, err error) {
	ctnMap := make(map[int64]entity.Container)
	ctnIDs := make([]int64, 0)

	imageIDMap := make(map[int64]bool)

	for _, container := range containers {
		ctnMap[container.ID] = container
		ctnIDs = append(ctnIDs, container.ID)
		imageIDMap[container.ImageID] = true
	}

	imageMap := make(map[int64]entity.Image)
	imageIDs := make([]int64, 0)
	for imageID := range imageIDMap {
		imageIDs = append(imageIDs, imageID)
	}

	images, err := c.ImageService.FindByID(imageIDs)

	for _, image := range images {
		imageMap[image.ID] = image
	}

	// mixin image -> container
	for index, ctn := range ctnMap {
		image := imageMap[ctn.ImageID]
		ctn.Image = &image
		ctnMap[index] = ctn
	}

	// mixin nat -> container
	nats, _ := c.NatRepository.FindByContainerID(ctnIDs)
	for _, nat := range nats {
		ctn := ctnMap[nat.ContainerID]
		ctn.Nats = append(ctn.Nats, nat)
		ctnMap[nat.ContainerID] = ctn
	}

	for _, ctn := range ctnMap {
		ctns = append(ctns, ctn)
	}

	return ctns, err
}

func (c *ContainerServiceImpl) FindByPodID(podIDs []int64) (containers []entity.Container, err error) {
	ctns, err := c.ContainerRepository.FindByPodID(podIDs)
	containers, err = c.Mixin(ctns)
	return containers, err
}
