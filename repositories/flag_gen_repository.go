package repositories

import (
	"github.com/elabosak233/pgshub/models/entity"
	"xorm.io/xorm"
)

type FlagGenRepository interface {
	Insert(flag entity.FlagGen) (f entity.FlagGen, err error)
}

type FlagGenRepositoryImpl struct {
	Db *xorm.Engine
}

func NewFlagGenRepositoryImpl(Db *xorm.Engine) FlagGenRepository {
	return &FlagGenRepositoryImpl{Db: Db}
}

func (t *FlagGenRepositoryImpl) Insert(flag entity.FlagGen) (f entity.FlagGen, err error) {
	_, err = t.Db.Table("flag_gen").Insert(&flag)
	return flag, err
}
