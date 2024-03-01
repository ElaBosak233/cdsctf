package service

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/repository"
)

type IImageService interface {
	FindByID(IDs []uint) (images []model.Image, err error)
	FindByChallengeID(challengeIDs []uint) (images []model.Image, err error)
}

type ImageServiceImpl struct {
	imageRepository repository.IImageRepository
	portRepository  repository.IPortRepository
	envRepository   repository.IEnvRepository
}

func NewImageServiceImpl(appRepository *repository.Repository) IImageService {
	return &ImageServiceImpl{
		imageRepository: appRepository.ImageRepository,
		portRepository:  appRepository.PortRepository,
		envRepository:   appRepository.EnvRepository,
	}
}

func (t *ImageServiceImpl) FindByID(IDs []uint) (images []model.Image, err error) {
	images, err = t.imageRepository.FindByID(IDs)
	return images, err
}

func (t *ImageServiceImpl) FindByChallengeID(challengeIDs []uint) (images []model.Image, err error) {
	images, err = t.imageRepository.FindByChallengeID(challengeIDs)
	return images, err
}
