package repository

import (
	model "github.com/elabosak233/pgshub/model/data"
	"xorm.io/xorm"
)

type ChallengeRepositoryImpl struct {
	Db *xorm.Engine
}

func NewChallengeRepositoryImpl(Db *xorm.Engine) ChallengeRepository {
	return &ChallengeRepositoryImpl{Db: Db}
}

// Insert implements ChallengeRepository
func (t *ChallengeRepositoryImpl) Insert(challenge model.Challenge) error {
	_, err := t.Db.Table("challenge").Insert(&challenge)
	return err
}

// Delete implements ChallengeRepository
func (t *ChallengeRepositoryImpl) Delete(id string) error {
	var user model.Challenge
	_, err := t.Db.Table("challenge").ID(id).Delete(&user)
	return err
}

// Update implements ChallengeRepository
func (t *ChallengeRepositoryImpl) Update(challenge model.Challenge) error {
	_, err := t.Db.Table("challenge").ID(challenge.ChallengeId).Update(&challenge)
	return err
}

// FindAll implements ChallengeRepository
func (t *ChallengeRepositoryImpl) FindAll() []model.Challenge {
	var challenge []model.Challenge
	err := t.Db.Table("challenge").Find(&challenge)
	if err != nil {
		return nil
	}
	return challenge
}

// FindById implements UserRepository
func (t *ChallengeRepositoryImpl) FindById(id string) (model.Challenge, error) {
	var challenge model.Challenge
	has, err := t.Db.Table("challenge").ID(id).Get(&challenge)
	if has {
		return challenge, nil
	} else {
		return challenge, err
	}
}
