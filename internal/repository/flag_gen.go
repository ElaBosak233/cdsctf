package repository

import (
	"github.com/elabosak233/pgshub/internal/model"
	"xorm.io/xorm"
)

type IFlagGenRepository interface {
	Insert(flag model.FlagGen) (f model.FlagGen, err error)
	FindByID(IDs []int64) (flags []model.FlagGen, err error)
	FindByPodID(podIDs []int64) (flags []model.FlagGen, err error)
}

type FlagGenRepository struct {
	Db *xorm.Engine
}

func NewFlagGenRepository(Db *xorm.Engine) IFlagGenRepository {
	return &FlagGenRepository{Db: Db}
}

func (t *FlagGenRepository) Insert(flag model.FlagGen) (f model.FlagGen, err error) {
	_, err = t.Db.Table("flag_gen").Insert(&flag)
	return flag, err
}

func (t *FlagGenRepository) FindByID(IDs []int64) (flags []model.FlagGen, err error) {
	err = t.Db.Table("flag_gen").In("id", IDs).Find(&flags)
	return flags, err
}

func (t *FlagGenRepository) FindByPodID(podIDs []int64) (flags []model.FlagGen, err error) {
	err = t.Db.Table("flag_gen").In("pod_id", podIDs).Find(&flags)
	return flags, err
}
