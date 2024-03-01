package repository

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"gorm.io/gorm"
)

type IImageRepository interface {
	Insert(image model.Image) (i model.Image, err error)
	Update(image model.Image) (i model.Image, err error)
	Delete(image model.Image) (err error)
	FindByID(IDs []uint) (images []model.Image, err error)
	FindByChallengeID(challengeIDs []uint) (images []model.Image, err error)
}

type ImageRepository struct {
	db *gorm.DB
}

func NewImageRepository(db *gorm.DB) IImageRepository {
	return &ImageRepository{db: db}
}

func (t *ImageRepository) Insert(image model.Image) (i model.Image, err error) {
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

func (t *ImageRepository) FindByID(IDs []uint) (images []model.Image, err error) {
	result := t.db.Table("images").
		Where("id IN ?", IDs).
		Preload("Ports").
		Preload("Envs").
		Find(&images)
	return images, result.Error
}

func (t *ImageRepository) FindByChallengeID(challengeIDs []uint) (images []model.Image, err error) {
	result := t.db.Table("images").
		Where("challenge_id IN ?", challengeIDs).
		Preload("Ports").
		Preload("Envs").
		Find(&images)
	return images, result.Error
}
