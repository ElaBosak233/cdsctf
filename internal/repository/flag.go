package repository

import (
	"github.com/elabosak233/pgshub/internal/model"
	"gorm.io/gorm"
)

type IFlagRepository interface {
	Insert(flag model.Flag) (f model.Flag, err error)
	Update(flag model.Flag) (f model.Flag, err error)
	Delete(flag model.Flag) (err error)
	FindByChallengeID(challengeIDs []uint) (flags []model.Flag, err error)
	DeleteByChallengeID(challengeIDs []uint) (err error)
}

type FlagRepository struct {
	Db *gorm.DB
}

func NewFlagRepository(Db *gorm.DB) IFlagRepository {
	return &FlagRepository{Db: Db}
}

func (t *FlagRepository) Insert(flag model.Flag) (f model.Flag, err error) {
	result := t.Db.Table("flags").Create(&flag)
	return flag, result.Error
}

func (t *FlagRepository) Update(flag model.Flag) (f model.Flag, err error) {
	result := t.Db.Table("flags").Model(&flag).Updates(&flag)
	return flag, result.Error
}

func (t *FlagRepository) Delete(flag model.Flag) (err error) {
	result := t.Db.Table("flags").Delete(&flag)
	return result.Error
}

func (t *FlagRepository) FindByChallengeID(challengeIDs []uint) (flags []model.Flag, err error) {
	result := t.Db.Table("flags").
		Where("challenge_id IN ?", challengeIDs).
		Find(&flags)
	return flags, result.Error
}

func (t *FlagRepository) DeleteByChallengeID(challengeIDs []uint) (err error) {
	result := t.Db.Table("flags").
		Where("challenge_id IN ?", challengeIDs).
		Delete(&model.Flag{})
	return result.Error
}
