package repository

import (
	"github.com/elabosak233/pgshub/internal/model"
	"github.com/elabosak233/pgshub/internal/model/dto/request"
	"gorm.io/gorm"
)

type ICategoryRepository interface {
	Create(category model.Category) (err error)
	Update(category model.Category) (err error)
	Find(req request.CategoryFindRequest) (categories []model.Category, err error)
	FindByID(IDs []uint) (categories []model.Category, err error)
}

type CategoryRepository struct {
	Db *gorm.DB
}

func NewCategoryRepositoryImpl(Db *gorm.DB) ICategoryRepository {
	return &CategoryRepository{Db: Db}
}

func (t *CategoryRepository) Create(category model.Category) (err error) {
	result := t.Db.Table("categories").Create(&category)
	return result.Error
}

func (t *CategoryRepository) Update(category model.Category) (err error) {
	result := t.Db.Table("categories").Updates(&category)
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
	result := applyFilters(t.Db.Table("categories")).Find(&categories)
	return categories, result.Error
}

func (t *CategoryRepository) FindByID(IDs []uint) (categories []model.Category, err error) {
	result := t.Db.Table("categories").Where("id IN ?", IDs).Find(&categories)
	return categories, result.Error
}
