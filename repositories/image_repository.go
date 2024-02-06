package repositories

import (
	"github.com/elabosak233/pgshub/models/entity"
	"xorm.io/xorm"
)

type ImageRepository interface {
	Insert(image entity.Image) (i entity.Image, err error)
	Update(image entity.Image) (i entity.Image, err error)
	Delete(image entity.Image) (err error)
	FindByChallengeID(challengeIDs []int64) (images []entity.Image, err error)
	DeleteByChallengeID(challengeIDs []int64) (err error)
}

type ImageRepositoryImpl struct {
	Db *xorm.Engine
}

func NewImageRepositoryImpl(Db *xorm.Engine) ImageRepository {
	return &ImageRepositoryImpl{Db: Db}
}

func (t *ImageRepositoryImpl) Insert(image entity.Image) (i entity.Image, err error) {
	_, err = t.Db.Table("image").Insert(&image)
	return image, err
}

func (t *ImageRepositoryImpl) Update(image entity.Image) (i entity.Image, err error) {
	_, err = t.Db.Table("image").ID(image.ImageID).Update(&image)
	return image, err
}

func (t *ImageRepositoryImpl) Delete(image entity.Image) (err error) {
	_, err = t.Db.Table("image").ID(image.ImageID).Delete(&image)
	return err
}

func (t *ImageRepositoryImpl) FindByChallengeID(challengeIDs []int64) (images []entity.Image, err error) {
	err = t.Db.Table("image").In("challenge_id", challengeIDs).Find(&images)
	return images, err
}

func (t *ImageRepositoryImpl) DeleteByChallengeID(challengeIDs []int64) (err error) {
	_, err = t.Db.Table("image").In("challenge_id", challengeIDs).Delete(&entity.Image{})
	return err
}
