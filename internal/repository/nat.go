package repository

import (
	"github.com/elabosak233/pgshub/internal/model"
	"xorm.io/xorm"
)

type INatRepository interface {
	Insert(nat model.Nat) (n model.Nat, err error)
	FindByInstanceID(instanceIDs []int64) (nats []model.Nat, err error)
}

type NatRepository struct {
	Db *xorm.Engine
}

func NewNatRepository(Db *xorm.Engine) INatRepository {
	return &NatRepository{Db: Db}
}

func (t *NatRepository) Insert(nat model.Nat) (n model.Nat, err error) {
	_, err = t.Db.Table("nat").Insert(&nat)
	return nat, err
}

func (t *NatRepository) FindByInstanceID(instanceIDs []int64) (nats []model.Nat, err error) {
	err = t.Db.Table("nat").In("instance_id", instanceIDs).Find(&nats)
	return nats, err
}
