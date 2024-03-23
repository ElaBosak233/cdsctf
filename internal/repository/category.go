package repository

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"gorm.io/gorm"
)

type ICategoryRepository interface {
	Create(category model.Category) (err error)
	Update(category model.Category) (err error)
	Find(req request.CategoryFindRequest) (categories []model.Category, err error)
	Delete(id uint) (err error)
}

type CategoryRepository struct {
	db *gorm.DB
}

func NewCategoryRepositoryImpl(db *gorm.DB) ICategoryRepository {
	return &CategoryRepository{db: db}
}

func (t *CategoryRepository) Create(category model.Category) (err error) {
	result := t.db.Table("categories").Create(&category)
	return result.Error
}

func (t *CategoryRepository) Update(category model.Category) (err error) {
	result := t.db.Table("categories").Updates(&category)
	return result.Error
}

func (t *CategoryRepository) Find(req request.CategoryFindRequest) (categories []model.Category, err error) {
	applyFilters := func(db *gorm.DB) *gorm.DB {
		if req.ID != 0 {
			db = db.Where("id = ?", req.ID)
		}
		if req.Name != "" {
			db = db.Where("name = ?", req.Name)
		}
		return db
	}
	result := applyFilters(t.db.Table("categories")).Find(&categories)
	return categories, result.Error
}

func (t *CategoryRepository) Delete(id uint) (err error) {
	result := t.db.Table("categories").Delete(&model.Category{
		ID: id,
	})
	return result.Error
}
