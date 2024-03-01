package repository

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"gorm.io/gorm"
)

type INatRepository interface {
	Insert(nat model.Nat) (n model.Nat, err error)
	FindByInstanceID(instanceIDs []uint) (nats []model.Nat, err error)
}

type NatRepository struct {
	db *gorm.DB
}

func NewNatRepository(db *gorm.DB) INatRepository {
	return &NatRepository{db: db}
}

func (t *NatRepository) Insert(nat model.Nat) (n model.Nat, err error) {
	result := t.db.Table("nats").Create(&nat)
	return nat, result.Error
}

func (t *NatRepository) FindByInstanceID(instanceIDs []uint) (nats []model.Nat, err error) {
	result := t.db.Table("nats").Where("instance_id IN ?", instanceIDs).Find(&nats)
	return nats, result.Error
}
