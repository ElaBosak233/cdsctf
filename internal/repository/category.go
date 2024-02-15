package repository

import (
	"github.com/elabosak233/pgshub/internal/model"
	"gorm.io/gorm"
)

type ICategoryRepository interface {
	Create(category model.Category) (err error)
	FindByID(IDs []uint) (categories []model.Category, err error)
}

type CategoryRepository struct {
	Db *gorm.DB
}

func NewCategoryRepositoryImpl(Db *gorm.DB) ICategoryRepository {
	return &CategoryRepository{Db: Db}
}

func (t *CategoryRepository) Create(category model.Category) (err error) {
	result := t.Db.Create(&category)
	return result.Error
}

func (t *CategoryRepository) FindByID(IDs []uint) (categories []model.Category, err error) {
	result := t.Db.Where("(id) IN ?", IDs).Find(&categories)
	return categories, result.Error
}
