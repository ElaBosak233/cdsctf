package repository

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"gorm.io/gorm"
)

type INatRepository interface {
	Create(nat model.Nat) (model.Nat, error)
}

type NatRepository struct {
	db *gorm.DB
}

func NewNatRepository(db *gorm.DB) INatRepository {
	return &NatRepository{db: db}
}

func (t *NatRepository) Create(nat model.Nat) (model.Nat, error) {
	result := t.db.Table("nats").Create(&nat)
	return nat, result.Error
}
