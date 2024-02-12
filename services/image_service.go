package services

import (
	"github.com/elabosak233/pgshub/models/entity"
	"github.com/elabosak233/pgshub/repositories"
)

type ImageService interface {
	FindByID(IDs []int64) (images []entity.Image, err error)
	FindByChallengeID(challengeIDs []int64) (images []entity.Image, err error)
}

type ImageServiceImpl struct {
	MixinService    MixinService
	ImageRepository repositories.ImageRepository
	PortRepository  repositories.PortRepository
	EnvRepository   repositories.EnvRepository
}

func NewImageServiceImpl(appRepository *repositories.Repositories) ImageService {
	return &ImageServiceImpl{
		MixinService:    NewMixinServiceImpl(appRepository),
		ImageRepository: appRepository.ImageRepository,
		PortRepository:  appRepository.PortRepository,
		EnvRepository:   appRepository.EnvRepository,
	}
}

func (t *ImageServiceImpl) FindByID(IDs []int64) (images []entity.Image, err error) {
	images, err = t.ImageRepository.FindByID(IDs)
	images, err = t.MixinService.MixinImage(images)
	return images, err
}

func (t *ImageServiceImpl) FindByChallengeID(challengeIDs []int64) (images []entity.Image, err error) {
	images, err = t.ImageRepository.FindByChallengeID(challengeIDs)
	images, err = t.MixinService.MixinImage(images)
	return images, err
}
