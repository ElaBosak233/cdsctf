package repository

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"gorm.io/gorm"
)

type IContainerRepository interface {
	Create(instance model.Container) (i model.Container, err error)
}

type ContainerRepository struct {
	db *gorm.DB
}

func NewContainerRepository(db *gorm.DB) IContainerRepository {
	return &ContainerRepository{db: db}
}

func (t *ContainerRepository) Create(container model.Container) (i model.Container, err error) {
	result := t.db.Table("containers").Create(&container)
	return container, result.Error
}
