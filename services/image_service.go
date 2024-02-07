package services

import (
	"github.com/elabosak233/pgshub/models/entity"
	"github.com/elabosak233/pgshub/repositories"
)

type ImageService interface {
	FindByChallengeID(challengeIDs []int64) (images []entity.Image, err error)
}

type ImageServiceImpl struct {
	ImageRepository repositories.ImageRepository
	PortRepository  repositories.PortRepository
	EnvRepository   repositories.EnvRepository
}

func NewImageServiceImpl(appRepository *repositories.Repositories) ImageService {
	return &ImageServiceImpl{
		ImageRepository: appRepository.ImageRepository,
		PortRepository:  appRepository.PortRepository,
		EnvRepository:   appRepository.EnvRepository,
	}
}

func (t *ImageServiceImpl) Mixin(images []entity.Image) (imgs []entity.Image, err error) {
	imageMap := make(map[int64]entity.Image)
	for _, image := range images {
		imageMap[image.ImageID] = image
	}
	imageIDs := make([]int64, 0)
	for id := range imageMap {
		imageIDs = append(imageIDs, id)
	}
	// mixin env -> image
	envs, _ := t.EnvRepository.FindByImageID(imageIDs)
	for _, env := range envs {
		image := imageMap[env.ImageID]
		image.Envs = append(image.Envs, env)
		imageMap[env.ImageID] = image
	}

	// mixin port -> image
	ports, _ := t.PortRepository.FindByImageID(imageIDs)
	for _, port := range ports {
		image := imageMap[port.ImageID]
		image.Ports = append(image.Ports, port)
		imageMap[port.ImageID] = image
	}

	for _, image := range imageMap {
		imgs = append(imgs, image)
	}

	return imgs, err
}

func (t *ImageServiceImpl) FindByChallengeID(challengeIDs []int64) (images []entity.Image, err error) {
	images, err = t.ImageRepository.FindByChallengeID(challengeIDs)
	images, err = t.Mixin(images)
	return images, err
}
