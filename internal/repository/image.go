package repository

import (
	"github.com/elabosak233/pgshub/internal/model"
	"gorm.io/gorm"
)

type IImageRepository interface {
	Insert(image model.Image) (i model.Image, err error)
	Update(image model.Image) (i model.Image, err error)
	Delete(image model.Image) (err error)
	FindByID(IDs []uint) (images []model.Image, err error)
	FindByChallengeID(challengeIDs []uint) (images []model.Image, err error)
	DeleteByChallengeID(challengeIDs []uint) (err error)
}

type ImageRepository struct {
	Db *gorm.DB
}

func NewImageRepository(Db *gorm.DB) IImageRepository {
	return &ImageRepository{Db: Db}
}

func (t *ImageRepository) Insert(image model.Image) (i model.Image, err error) {
	result := t.Db.Table("images").Create(&image)
	return image, result.Error
}

func (t *ImageRepository) Update(image model.Image) (i model.Image, err error) {
	result := t.Db.Table("images").Model(&image).Updates(&image)
	return image, result.Error
}

func (t *ImageRepository) Delete(image model.Image) (err error) {
	result := t.Db.Table("images").Delete(&image)
	return result.Error
}

func (t *ImageRepository) FindByID(IDs []uint) (images []model.Image, err error) {
	result := t.Db.Table("images").Where("id IN ?", IDs).Find(&images)
	return images, result.Error
}

func (t *ImageRepository) FindByChallengeID(challengeIDs []uint) (images []model.Image, err error) {
	result := t.Db.Table("images").Where("challenge_id IN ?", challengeIDs).Find(&images)
	return images, result.Error
}

func (t *ImageRepository) DeleteByChallengeID(challengeIDs []uint) (err error) {
	result := t.Db.Table("images").Where("challenge_id IN ?", challengeIDs).Delete(&model.Image{})
	return result.Error
}
