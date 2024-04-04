package service

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"github.com/elabosak233/cloudsdale/internal/repository"
	"github.com/mitchellh/mapstructure"
)

type IImageService interface {
	Create(req request.ImageCreateRequest) (err error)
	Update(req request.ImageUpdateRequest) (err error)
	Delete(req request.ImageDeleteRequest) (err error)
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
	var image model.Image
	_ = mapstructure.Decode(req, &image)
	_, err = t.imageRepository.Create(image)
	return err
}

func (t *ImageService) Update(req request.ImageUpdateRequest) (err error) {
	var image model.Image
	_ = mapstructure.Decode(req, &image)
	_, err = t.imageRepository.Update(image)
	return
}

func (t *ImageService) Delete(req request.ImageDeleteRequest) (err error) {
	var image model.Image
	_ = mapstructure.Decode(req, &image)
	err = t.imageRepository.Delete(image)
	return
}
