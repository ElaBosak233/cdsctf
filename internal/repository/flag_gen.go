package repository

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"gorm.io/gorm"
)

type IFlagGenRepository interface {
	Create(flag model.FlagGen) (model.FlagGen, error)
	FindByPodID(podIDs []uint) ([]model.FlagGen, error)
}

type FlagGenRepository struct {
	db *gorm.DB
}

func NewFlagGenRepository(db *gorm.DB) IFlagGenRepository {
	return &FlagGenRepository{db: db}
}

func (t *FlagGenRepository) Create(flag model.FlagGen) (model.FlagGen, error) {
	result := t.db.Table("flag_gens").Create(&flag)
	return flag, result.Error
}

func (t *FlagGenRepository) FindByPodID(podIDs []uint) ([]model.FlagGen, error) {
	var flags []model.FlagGen
	result := t.db.Table("flag_gens").
		Where("pod_id IN ?", podIDs).
		Find(&flags)
	return flags, result.Error
}
