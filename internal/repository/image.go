package repository

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"gorm.io/gorm"
)

type IImageRepository interface {
	Create(image model.Image) (i model.Image, err error)
	Update(image model.Image) (i model.Image, err error)
	Delete(image model.Image) (err error)
}

type ImageRepository struct {
	db *gorm.DB
}

func NewImageRepository(db *gorm.DB) IImageRepository {
	return &ImageRepository{db: db}
}

func (t *ImageRepository) Create(image model.Image) (i model.Image, err error) {
	result := t.db.Table("images").Create(&image)
	return image, result.Error
}

func (t *ImageRepository) Update(image model.Image) (i model.Image, err error) {
	result := t.db.Table("images").Model(&image).Updates(&image)
	return image, result.Error
}

func (t *ImageRepository) Delete(image model.Image) (err error) {
	result := t.db.Table("images").Delete(&image)
	return result.Error
}
