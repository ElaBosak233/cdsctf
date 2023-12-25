package repositorys

import (
	model "github.com/elabosak233/pgshub/internal/models/data"
	"github.com/elabosak233/pgshub/internal/models/request"
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

func (t *ChallengeRepositoryImpl) Find(req request.ChallengeFindRequest) (challenges []model.Challenge, count int64, err error) {
	applyFilter := func(q *xorm.Session) *xorm.Session {
		if req.Category != "" {
			q = q.Where("category = ?", req.Category)
		}
		if req.Title != "" {
			q = q.Where("title LIKE ?", "%"+req.Title+"%")
		}
		if req.IsPracticable != -1 {
			if req.IsPracticable == 0 {
				q = q.Where("is_practicable = 0")
			} else if req.IsPracticable == 1 {
				q = q.Where("is_practicable = 1")
			}
		}
		if req.IsDynamic != -1 {
			if req.IsDynamic == 0 {
				q = q.Where("is_dynamic = 0")
			} else if req.IsDynamic == 1 {
				q = q.Where("is_dynamic = 1")
			}
		}
		if req.Difficulty != -1 {
			q = q.Where("difficulty = ?", req.Difficulty)
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
func (t *ChallengeRepositoryImpl) FindById(id string) (model.Challenge, error) {
	var challenge model.Challenge
	has, err := t.Db.Table("challenge").ID(id).Get(&challenge)
	if has {
		return challenge, nil
	} else {
		return challenge, err
	}
}
