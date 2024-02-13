package repository

import (
	"github.com/elabosak233/pgshub/internal/model"
	"xorm.io/xorm"
)

type ICategoryRepository interface {
	Create(category model.Category) (err error)
	FindByID(IDs []int64) (categories []model.Category, err error)
}

type CategoryRepository struct {
	Db *xorm.Engine
}

func NewCategoryRepositoryImpl(Db *xorm.Engine) ICategoryRepository {
	return &CategoryRepository{Db: Db}
}

func (t *CategoryRepository) Create(category model.Category) (err error) {
	_, err = t.Db.Insert(&category)
	return err
}

func (t *CategoryRepository) FindByID(IDs []int64) (categories []model.Category, err error) {
	err = t.Db.In("id", IDs).Find(&categories)
	return categories, err
}
