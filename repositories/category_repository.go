package repositories

import (
	"github.com/elabosak233/pgshub/models/entity"
	"xorm.io/xorm"
)

type CategoryRepository interface {
	Create(category entity.Category) (err error)
	FindByID(IDs []int64) (categories []entity.Category, err error)
}

type CategoryRepositoryImpl struct {
	Db *xorm.Engine
}

func NewCategoryRepositoryImpl(Db *xorm.Engine) CategoryRepository {
	return &CategoryRepositoryImpl{Db: Db}
}

func (t *CategoryRepositoryImpl) Create(category entity.Category) (err error) {
	_, err = t.Db.Insert(&category)
	return err
}

func (t *CategoryRepositoryImpl) FindByID(IDs []int64) (categories []entity.Category, err error) {
	err = t.Db.In("id", IDs).Find(&categories)
	return categories, err
}
