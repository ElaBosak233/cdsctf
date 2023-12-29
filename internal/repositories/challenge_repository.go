package repositories

import (
	model "github.com/elabosak233/pgshub/internal/models/data"
	"github.com/elabosak233/pgshub/internal/models/request"
	"xorm.io/xorm"
)

type ChallengeRepository interface {
	Insert(user model.Challenge) error
	Update(user model.Challenge) error
	Delete(id string) error
	FindById(id string, isDetailed int) (challenge model.Challenge, err error)
	Find(req request.ChallengeFindRequest) (challenges []model.Challenge, count int64, err error)
}

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

func (t *ChallengeRepositoryImpl) Find(req request.ChallengeFindRequest) (challenges []model.Challenge, count int64, err error) {
	applyFilter := func(q *xorm.Session) *xorm.Session {
		if req.Category != "" {
			q = q.Where("category = ?", req.Category)
		}
		if req.Title != "" {
			q = q.Where("title LIKE ?", "%"+req.Title+"%")
		}
		if req.IsPracticable != -1 {
			q = q.Where("is_practicable = ?", req.IsPracticable == 1)
		}
		if req.IsDynamic != -1 {
			q = q.Where("is_dynamic = ?", req.IsDynamic == 1)
		}
		if req.Difficulty != -1 {
			q = q.Where("difficulty = ?", req.Difficulty)
		}
		if req.IsDetailed == 0 {
			q = q.Omit("flag", "flag_fmt", "flag_env", "image")
		}
		return q
	}
	db := applyFilter(t.Db.Table("challenge"))
	ct := applyFilter(t.Db.Table("challenge"))
	count, err = ct.Count(&model.Challenge{})
	if req.Page != -1 && req.Size != -1 {
		offset := (req.Page - 1) * req.Size
		db = db.Limit(req.Size, offset)
	}
	err = db.Find(&challenges)
	return challenges, count, err
}

// FindAll implements ChallengeRepository
func (t *ChallengeRepositoryImpl) FindAll() []model.Challenge {
	var challenges []model.Challenge
	err := t.Db.Table("challenge").Find(&challenges)
	if err != nil {
		return nil
	}
	return challenges
}

// FindById implements ChallengeRepository
func (t *ChallengeRepositoryImpl) FindById(id string, isDetailed int) (challenge model.Challenge, err error) {
	db := t.Db.Table("challenge").ID(id)
	if isDetailed == 0 {
		db = db.Omit("flag", "flag_fmt", "flag_env", "image")
	}
	_, err = db.Get(&challenge)
	return challenge, err
}
