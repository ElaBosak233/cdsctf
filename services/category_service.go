package services

import (
	"github.com/elabosak233/pgshub/models/entity"
	"github.com/elabosak233/pgshub/repositories"
)

type CategoryService interface {
	Create(req entity.Category) (err error)
}

type CategoryServiceImpl struct {
	CategoryRepository repositories.CategoryRepository
}

func NewCategoryServiceImpl(appRepository *repositories.Repositories) CategoryService {
	return &CategoryServiceImpl{
		CategoryRepository: appRepository.CategoryRepository,
	}
}

func (c *CategoryServiceImpl) Create(req entity.Category) (err error) {
	return c.CategoryRepository.Create(req)
}
