package repository

import (
	"github.com/elabosak233/pgshub/internal/model"
	"xorm.io/xorm"
)

type IImageRepository interface {
	Insert(image model.Image) (i model.Image, err error)
	Update(image model.Image) (i model.Image, err error)
	Delete(image model.Image) (err error)
	FindByID(IDs []int64) (images []model.Image, err error)
	FindByChallengeID(challengeIDs []int64) (images []model.Image, err error)
	DeleteByChallengeID(challengeIDs []int64) (err error)
}

type ImageRepository struct {
	Db *xorm.Engine
}

func NewImageRepository(Db *xorm.Engine) IImageRepository {
	return &ImageRepository{Db: Db}
}

func (t *ImageRepository) Insert(image model.Image) (i model.Image, err error) {
	_, err = t.Db.Table("image").Insert(&image)
	return image, err
}

func (t *ImageRepository) Update(image model.Image) (i model.Image, err error) {
	_, err = t.Db.Table("image").ID(image.ID).Update(&image)
	return image, err
}

func (t *ImageRepository) Delete(image model.Image) (err error) {
	_, err = t.Db.Table("image").ID(image.ID).Delete(&image)
	return err
}

func (t *ImageRepository) FindByID(IDs []int64) (images []model.Image, err error) {
	err = t.Db.Table("image").In("id", IDs).Find(&images)
	return images, err
}

func (t *ImageRepository) FindByChallengeID(challengeIDs []int64) (images []model.Image, err error) {
	err = t.Db.Table("image").In("challenge_id", challengeIDs).Find(&images)
	return images, err
}

func (t *ImageRepository) DeleteByChallengeID(challengeIDs []int64) (err error) {
	_, err = t.Db.Table("image").In("challenge_id", challengeIDs).Delete(&model.Image{})
	return err
}
