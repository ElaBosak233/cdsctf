package repository

import (
	"github.com/elabosak233/pgshub/internal/model"
	"xorm.io/xorm"
)

type IFlagRepository interface {
	Insert(flag model.Flag) (f model.Flag, err error)
	Update(flag model.Flag) (f model.Flag, err error)
	Delete(flag model.Flag) (err error)
	FindByChallengeID(challengeIDs []int64) (flags []model.Flag, err error)
	DeleteByChallengeID(challengeIDs []int64) (err error)
}

type FlagRepository struct {
	Db *xorm.Engine
}

func NewFlagRepository(Db *xorm.Engine) IFlagRepository {
	return &FlagRepository{Db: Db}
}

func (t *FlagRepository) Insert(flag model.Flag) (f model.Flag, err error) {
	_, err = t.Db.Table("flag").Insert(&flag)
	return flag, err
}

func (t *FlagRepository) Update(flag model.Flag) (f model.Flag, err error) {
	_, err = t.Db.Table("flag").ID(flag.ID).Update(&flag)
	return flag, err
}

func (t *FlagRepository) Delete(flag model.Flag) (err error) {
	_, err = t.Db.Table("flag").ID(flag.ID).Delete(&flag)
	return err
}

func (t *FlagRepository) FindByChallengeID(challengeIDs []int64) (flags []model.Flag, err error) {
	err = t.Db.Table("flag").In("challenge_id", challengeIDs).Find(&flags)
	return flags, err
}

func (t *FlagRepository) DeleteByChallengeID(challengeIDs []int64) (err error) {
	_, err = t.Db.Table("flag").In("challenge_id", challengeIDs).Delete(&model.Flag{})
	return err
}
