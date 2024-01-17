package repositories

import (
	"fmt"
	model "github.com/elabosak233/pgshub/internal/models/data"
	"github.com/elabosak233/pgshub/internal/models/request"
	"github.com/elabosak233/pgshub/internal/models/response"
	"xorm.io/xorm"
)

type ChallengeRepository interface {
	Insert(user model.Challenge) error
	Update(user model.Challenge) error
	Delete(id int64) error
	FindById(id int64, isDetailed int) (challenge model.Challenge, err error)
	Find(req request.ChallengeFindRequest) (challenges []response.ChallengeResponse, count int64, err error)
}

type ChallengeRepositoryImpl struct {
	Db *xorm.Engine
}

func NewChallengeRepositoryImpl(Db *xorm.Engine) ChallengeRepository {
	return &ChallengeRepositoryImpl{Db: Db}
}

// Insert implements ChallengeRepository
func (t *ChallengeRepositoryImpl) Insert(challenge model.Challenge) error {
	_, err := t.Db.Table("challenges").Insert(&challenge)
	return err
}

// Delete implements ChallengeRepository
func (t *ChallengeRepositoryImpl) Delete(id int64) error {
	var challenge model.Challenge
	_, err := t.Db.Table("challenges").ID(id).Delete(&challenge)
	return err
}

// Update implements ChallengeRepository
func (t *ChallengeRepositoryImpl) Update(challenge model.Challenge) error {
	fmt.Println(challenge.ChallengeId)
	_, err := t.Db.Table("challenges").ID(challenge.ChallengeId).MustCols("is_practicable, is_dynamic, has_attachment").Update(&challenge)
	return err
}

func (t *ChallengeRepositoryImpl) Find(req request.ChallengeFindRequest) (challenges []response.ChallengeResponse, count int64, err error) {
	applyFilter := func(q *xorm.Session) *xorm.Session {
		if req.Category != "" {
			q = q.Where("category = ?", req.Category)
		}
		if req.Title != "" {
			q = q.Where("title LIKE ?", "%"+req.Title+"%")
		}
		if req.IsPracticable != 0 {
			q = q.Where("is_practicable = ?", req.IsPracticable == 1)
		}
		if req.IsDynamic != 0 {
			q = q.Where("is_dynamic = ?", req.IsDynamic == 1)
		}
		if req.Difficulty != 0 {
			q = q.Where("difficulty = ?", req.Difficulty)
		}
		return q
	}
	db := applyFilter(t.Db.Table("challenges AS c"))
	ct := applyFilter(t.Db.Table("challenges"))
	count, err = ct.Count(&model.Challenge{})
	if len(req.SortBy) > 0 {
		sortKey := req.SortBy[0]
		sortOrder := req.SortBy[1]
		if sortOrder == "asc" {
			db = db.Asc("challenges." + sortKey)
		} else if sortOrder == "desc" {
			db = db.Desc("challenges." + sortKey)
		}
	}
	if req.Page != 0 && req.Size > 0 {
		offset := (req.Page - 1) * req.Size
		db = db.Limit(req.Size, offset)
	}
	// TODO 这里还需要判断是练习场还是比赛，如果是比赛，需要判断 team_id 和 game_id
	db = db.Join("LEFT", "submissions", "submissions.challenge_id = c.id AND submissions.status = 2 AND submissions.user_id = ?", req.UserId)
	db = db.Cols("submissions.id as is_solved")
	if req.IsDetailed == 0 {
		db = db.Cols("c.id", "c.title", "c.description",
			"c.category", "c.duration", "c.is_dynamic",
			"c.has_attachment", "c.difficulty", "c.practice_pts",
		)
	} else {
		db = db.Cols("c.*")
	}
	err = db.Find(&challenges)
	return challenges, count, err
}

// FindAll implements ChallengeRepository
func (t *ChallengeRepositoryImpl) FindAll() []model.Challenge {
	var challenges []model.Challenge
	err := t.Db.Table("challenges").Find(&challenges)
	if err != nil {
		return nil
	}
	return challenges
}

// FindById implements ChallengeRepository
func (t *ChallengeRepositoryImpl) FindById(id int64, isDetailed int) (challenge model.Challenge, err error) {
	db := t.Db.Table("challenges").ID(id)
	if isDetailed == 0 {
		db = db.Omit("flag", "flag_fmt", "flag_env", "image")
	}
	_, err = db.Get(&challenge)
	return challenge, err
}
