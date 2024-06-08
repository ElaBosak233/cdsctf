package repository

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"gorm.io/gorm"
)

type IPortRepository interface {
	Create(port model.Port) (model.Port, error)
	Update(port model.Port) (model.Port, error)
	Delete(port model.Port) error
}

type PortRepository struct {
	db *gorm.DB
}

func NewPortRepository(db *gorm.DB) IPortRepository {
	return &PortRepository{db: db}
}

func (t *PortRepository) Create(port model.Port) (model.Port, error) {
	result := t.db.Table("ports").Create(&port)
	return port, result.Error
}

func (t *PortRepository) Update(port model.Port) (model.Port, error) {
	result := t.db.Table("ports").Model(&port).Updates(&port)
	return port, result.Error
}

func (t *PortRepository) Delete(port model.Port) error {
	result := t.db.Table("ports").Delete(&port)
	return result.Error
}
