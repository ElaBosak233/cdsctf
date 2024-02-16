package service

import (
	"github.com/elabosak233/pgshub/internal/model"
	"github.com/elabosak233/pgshub/internal/model/dto/request"
	"github.com/elabosak233/pgshub/internal/repository"
)

type ICategoryService interface {
	Create(req model.Category) (err error)
	Update(req model.Category) (err error)
	Find(req request.CategoryFindRequest) (categories []model.Category, err error)
}

type CategoryService struct {
	CategoryRepository repository.ICategoryRepository
}

func NewCategoryService(appRepository *repository.Repository) ICategoryService {
	return &CategoryService{
		CategoryRepository: appRepository.CategoryRepository,
	}
}

func (c *CategoryService) Create(req model.Category) (err error) {
	return c.CategoryRepository.Create(req)
}

func (c *CategoryService) Update(req model.Category) (err error) {
	return c.CategoryRepository.Update(req)
}

func (c *CategoryService) Find(req request.CategoryFindRequest) (categories []model.Category, err error) {
	return c.CategoryRepository.Find(req)
}
