package repository

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"gorm.io/gorm"
)

type IInstanceRepository interface {
	Insert(instance model.Instance) (i model.Instance, err error)
}

type InstanceRepository struct {
	db *gorm.DB
}

func NewInstanceRepository(db *gorm.DB) IInstanceRepository {
	return &InstanceRepository{db: db}
}

func (t *InstanceRepository) Insert(instance model.Instance) (i model.Instance, err error) {
	result := t.db.Table("instances").Create(&instance)
	return instance, result.Error
}
