package repositories

import (
	"github.com/elabosak233/pgshub/models/entity"
	"xorm.io/xorm"
)

type FlagRepository interface {
	Insert(flag entity.Flag) (f entity.Flag, err error)
	Update(flag entity.Flag) (f entity.Flag, err error)
	Delete(flag entity.Flag) (err error)
	FindByChallengeID(challengeIDs []int64) (flags []entity.Flag, err error)
	DeleteByChallengeID(challengeIDs []int64) (err error)
}

type FlagRepositoryImpl struct {
	Db *xorm.Engine
}

func NewFlagRepositoryImpl(Db *xorm.Engine) FlagRepository {
	return &FlagRepositoryImpl{Db: Db}
}

func (t *FlagRepositoryImpl) Insert(flag entity.Flag) (f entity.Flag, err error) {
	_, err = t.Db.Table("flag").Insert(&flag)
	return flag, err
}

func (t *FlagRepositoryImpl) Update(flag entity.Flag) (f entity.Flag, err error) {
	_, err = t.Db.Table("flag").ID(flag.FlagID).Update(&flag)
	return flag, err
}

func (t *FlagRepositoryImpl) Delete(flag entity.Flag) (err error) {
	_, err = t.Db.Table("flag").ID(flag.FlagID).Delete(&flag)
	return err
}

func (t *FlagRepositoryImpl) FindByChallengeID(challengeIDs []int64) (flags []entity.Flag, err error) {
	err = t.Db.Table("flag").In("challenge_id", challengeIDs).Find(&flags)
	return flags, err
}

func (t *FlagRepositoryImpl) DeleteByChallengeID(challengeIDs []int64) (err error) {
	_, err = t.Db.Table("flag").In("challenge_id", challengeIDs).Delete(&entity.Flag{})
	return err
}
