package service

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/request"
	"github.com/elabosak233/cloudsdale/internal/repository"
)

type ICategoryService interface {
	// Create will create a new category with the given request.
	Create(req model.Category) error

	// Update will update the category with the given request.
	Update(req model.Category) error

	// Find will find the category with the given request, and return the categories.
	Find(req request.CategoryFindRequest) ([]model.Category, error)

	// Delete will delete the category with the given request.
	Delete(req request.CategoryDeleteRequest) error
}

type CategoryService struct {
	categoryRepository repository.ICategoryRepository
}

func NewCategoryService(r *repository.Repository) ICategoryService {
	return &CategoryService{
		categoryRepository: r.CategoryRepository,
	}
}

func (c *CategoryService) Create(req model.Category) error {
	return c.categoryRepository.Create(req)
}

func (c *CategoryService) Update(req model.Category) error {
	return c.categoryRepository.Update(req)
}

func (c *CategoryService) Find(req request.CategoryFindRequest) ([]model.Category, error) {
	return c.categoryRepository.Find(req)
}

func (c *CategoryService) Delete(req request.CategoryDeleteRequest) error {
	return c.categoryRepository.Delete(req.ID)
}
