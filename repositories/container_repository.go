package repositories

import (
	"github.com/elabosak233/pgshub/models/entity"
	"xorm.io/xorm"
)

type ContainerRepository interface {
	Insert(container entity.Container) (c entity.Container, err error)
	FindByPodID(podIDs []int64) (containers []entity.Container, err error)
}

type ContainerRepositoryImpl struct {
	Db *xorm.Engine
}

func NewContainerRepositoryImpl(Db *xorm.Engine) ContainerRepository {
	return &ContainerRepositoryImpl{Db: Db}
}

func (t *ContainerRepositoryImpl) Insert(container entity.Container) (c entity.Container, err error) {
	_, err = t.Db.Table("container").Insert(&container)
	return container, err
}

func (t *ContainerRepositoryImpl) FindByPodID(podIDs []int64) (containers []entity.Container, err error) {
	err = t.Db.Table("container").In("pod_id", podIDs).Find(&containers)
	return containers, err
}
