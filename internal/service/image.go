package service

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"github.com/elabosak233/cloudsdale/internal/repository"
)

type IImageService interface {
	Create(req request.ImageCreateRequest) (err error)
	Update(req request.ImageUpdateRequest) (err error)
	Delete(req request.ImageDeleteRequest) (err error)
	FindByID(IDs []uint) (images []model.Image, err error)
	FindByChallengeID(challengeIDs []uint) (images []model.Image, err error)
}

type ImageService struct {
	imageRepository repository.IImageRepository
	portRepository  repository.IPortRepository
	envRepository   repository.IEnvRepository
}

func NewImageService(appRepository *repository.Repository) IImageService {
	return &ImageService{
		imageRepository: appRepository.ImageRepository,
		portRepository:  appRepository.PortRepository,
		envRepository:   appRepository.EnvRepository,
	}
}

func (t *ImageService) Create(req request.ImageCreateRequest) (err error) {
	//TODO implement me
	panic("implement me")
}

func (t *ImageService) Update(req request.ImageUpdateRequest) (err error) {
	//TODO implement me
	panic("implement me")
}

func (t *ImageService) Delete(req request.ImageDeleteRequest) (err error) {
	//TODO implement me
	panic("implement me")
}

func (t *ImageService) FindByID(IDs []uint) (images []model.Image, err error) {
	images, err = t.imageRepository.FindByID(IDs)
	return images, err
}

func (t *ImageService) FindByChallengeID(challengeIDs []uint) (images []model.Image, err error) {
	images, err = t.imageRepository.FindByChallengeID(challengeIDs)
	return images, err
}
