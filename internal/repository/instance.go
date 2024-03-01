package repository

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"gorm.io/gorm"
)

type IInstanceRepository interface {
	Insert(container model.Instance) (c model.Instance, err error)
	FindByPodID(podIDs []uint) (containers []model.Instance, err error)
}

type InstanceRepository struct {
	db *gorm.DB
}

func NewInstanceRepository(db *gorm.DB) IInstanceRepository {
	return &InstanceRepository{db: db}
}

func (t *InstanceRepository) Insert(container model.Instance) (c model.Instance, err error) {
	result := t.db.Table("instances").Create(&container)
	return container, result.Error
}

func (t *InstanceRepository) FindByPodID(podIDs []uint) (containers []model.Instance, err error) {
	result := t.db.Table("instances").
		Where("pod_id IN ?", podIDs).
		Preload("Image").
		Preload("Nats").
		Find(&containers)
	return containers, result.Error
}
