package repository

import (
	"github.com/elabosak233/pgshub/internal/model"
	"xorm.io/xorm"
)

type IInstanceRepository interface {
	Insert(container model.Instance) (c model.Instance, err error)
	FindByPodID(podIDs []int64) (containers []model.Instance, err error)
}

type InstanceRepository struct {
	Db *xorm.Engine
}

func NewInstanceRepository(Db *xorm.Engine) IInstanceRepository {
	return &InstanceRepository{Db: Db}
}

func (t *InstanceRepository) Insert(container model.Instance) (c model.Instance, err error) {
	_, err = t.Db.Table("instance").Insert(&container)
	return container, err
}

func (t *InstanceRepository) FindByPodID(podIDs []int64) (containers []model.Instance, err error) {
	err = t.Db.Table("instance").In("pod_id", podIDs).Find(&containers)
	return containers, err
}
