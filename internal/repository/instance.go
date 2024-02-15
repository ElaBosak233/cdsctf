package repository

import (
	"github.com/elabosak233/pgshub/internal/model"
	"gorm.io/gorm"
)

type IInstanceRepository interface {
	Insert(container model.Instance) (c model.Instance, err error)
	FindByPodID(podIDs []uint) (containers []model.Instance, err error)
}

type InstanceRepository struct {
	Db *gorm.DB
}

func NewInstanceRepository(Db *gorm.DB) IInstanceRepository {
	return &InstanceRepository{Db: Db}
}

func (t *InstanceRepository) Insert(container model.Instance) (c model.Instance, err error) {
	result := t.Db.Table("instances").Create(&container)
	return container, result.Error
}

func (t *InstanceRepository) FindByPodID(podIDs []uint) (containers []model.Instance, err error) {
	result := t.Db.Table("instances").
		Where("pod_id IN ?", podIDs).
		Preload("Image").
		Preload("Nats").
		Find(&containers)
	return containers, result.Error
}
