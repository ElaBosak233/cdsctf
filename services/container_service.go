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
	challengeMap := make(map[int64]bool)

	for _, ctn := range containers {
		ctnMap[ctn.ContainerID] = ctn
		ctnIDs = append(ctnIDs, ctn.ContainerID)
		challengeMap[ctn.ChallengeID] = true
	}

	challengeIDs := make([]int64, 0)
	for id := range challengeMap {
		challengeIDs = append(challengeIDs, id)
	}

	imageMap := make(map[int64]entity.Image)
	images, _ := c.ImageService.FindByChallengeID(challengeIDs)
	for _, image := range images {
		imageMap[image.ImageID] = image
	}

	// mixin image -> container
	for index, ctn := range ctnMap {
		image := imageMap[ctn.ImageID]
		ctn.Image = image
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
