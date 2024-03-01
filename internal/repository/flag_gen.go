package repository

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"gorm.io/gorm"
)

type IFlagGenRepository interface {
	Insert(flag model.FlagGen) (f model.FlagGen, err error)
	FindByID(IDs []uint) (flags []model.FlagGen, err error)
	FindByPodID(podIDs []uint) (flags []model.FlagGen, err error)
}

type FlagGenRepository struct {
	db *gorm.DB
}

func NewFlagGenRepository(db *gorm.DB) IFlagGenRepository {
	return &FlagGenRepository{db: db}
}

func (t *FlagGenRepository) Insert(flag model.FlagGen) (f model.FlagGen, err error) {
	result := t.db.Table("flag_gens").Create(&flag)
	return flag, result.Error
}

func (t *FlagGenRepository) FindByID(IDs []uint) (flags []model.FlagGen, err error) {
	result := t.db.Table("flag_gens").
		Where("id IN ?", IDs).
		Find(&flags)
	return flags, result.Error
}

func (t *FlagGenRepository) FindByPodID(podIDs []uint) (flags []model.FlagGen, err error) {
	result := t.db.Table("flag_gens").
		Where("pod_id IN ?", podIDs).
		Find(&flags)
	return flags, result.Error
}
