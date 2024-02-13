package service

import (
	"github.com/elabosak233/pgshub/internal/model"
	"github.com/elabosak233/pgshub/internal/repository"
)

type IImageService interface {
	FindByID(IDs []int64) (images []model.Image, err error)
	FindByChallengeID(challengeIDs []int64) (images []model.Image, err error)
}

type ImageServiceImpl struct {
	MixinService    IMixinService
	ImageRepository repository.IImageRepository
	PortRepository  repository.IPortRepository
	EnvRepository   repository.IEnvRepository
}

func NewImageServiceImpl(appRepository *repository.Repository) IImageService {
	return &ImageServiceImpl{
		MixinService:    NewMixinService(appRepository),
		ImageRepository: appRepository.ImageRepository,
		PortRepository:  appRepository.PortRepository,
		EnvRepository:   appRepository.EnvRepository,
	}
}

func (t *ImageServiceImpl) FindByID(IDs []int64) (images []model.Image, err error) {
	images, err = t.ImageRepository.FindByID(IDs)
	images, err = t.MixinService.MixinImage(images)
	return images, err
}

func (t *ImageServiceImpl) FindByChallengeID(challengeIDs []int64) (images []model.Image, err error) {
	images, err = t.ImageRepository.FindByChallengeID(challengeIDs)
	images, err = t.MixinService.MixinImage(images)
	return images, err
}
