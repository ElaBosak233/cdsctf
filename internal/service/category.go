package service

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"github.com/elabosak233/cloudsdale/internal/repository"
)

type ICategoryService interface {
	Create(req model.Category) (err error)
	Update(req model.Category) (err error)
	Find(req request.CategoryFindRequest) (categories []model.Category, err error)
}

type CategoryService struct {
	categoryRepository repository.ICategoryRepository
}

func NewCategoryService(appRepository *repository.Repository) ICategoryService {
	return &CategoryService{
		categoryRepository: appRepository.CategoryRepository,
	}
}

func (c *CategoryService) Create(req model.Category) (err error) {
	return c.categoryRepository.Create(req)
}

func (c *CategoryService) Update(req model.Category) (err error) {
	return c.categoryRepository.Update(req)
}

func (c *CategoryService) Find(req request.CategoryFindRequest) (categories []model.Category, err error) {
	return c.categoryRepository.Find(req)
}
