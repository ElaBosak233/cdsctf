package repositories

import (
	"github.com/elabosak233/pgshub/models/entity"
	"xorm.io/xorm"
)

type NatRepository interface {
	Insert(nat entity.Nat) (n entity.Nat, err error)
	FindByContainerID(containerIDs []int64) (nats []entity.Nat, err error)
}

type NatRepositoryImpl struct {
	Db *xorm.Engine
}

func NewNatRepositoryImpl(Db *xorm.Engine) NatRepository {
	return &NatRepositoryImpl{Db: Db}
}

func (t *NatRepositoryImpl) Insert(nat entity.Nat) (n entity.Nat, err error) {
	_, err = t.Db.Table("nat").Insert(&nat)
	return nat, err
}

func (t *NatRepositoryImpl) FindByContainerID(containerIDs []int64) (nats []entity.Nat, err error) {
	err = t.Db.Table("nat").In("container_id", containerIDs).Find(&nats)
	return nats, err
}
