package repositories

import (
	"fmt"
	"github.com/elabosak233/pgshub/models/entity"
	"github.com/elabosak233/pgshub/models/request"
	"github.com/elabosak233/pgshub/models/response"
	challengeValidator "github.com/elabosak233/pgshub/utils/validator/challenge"
	"xorm.io/xorm"
)

type ChallengeRepository interface {
	Insert(user entity.Challenge) error
	Update(user entity.Challenge) error
	Delete(id int64) error
	FindById(id int64, isDetailed int) (challenge entity.Challenge, err error)
	Find(req request.ChallengeFindRequest) (challenges []response.ChallengeResponse, count int64, err error)
}

type ChallengeRepositoryImpl struct {
	Db *xorm.Engine
}

func NewChallengeRepositoryImpl(Db *xorm.Engine) ChallengeRepository {
	return &ChallengeRepositoryImpl{Db: Db}
}

func (t *ChallengeRepositoryImpl) Insert(challenge entity.Challenge) error {
	_, err := t.Db.Table("challenge").Insert(&challenge)
	return err
}

func (t *ChallengeRepositoryImpl) Delete(id int64) error {
	var challenge entity.Challenge
	_, err := t.Db.Table("challenge").ID(id).Delete(&challenge)
	return err
}

func (t *ChallengeRepositoryImpl) Update(challenge entity.Challenge) error {
	fmt.Println(challenge.ChallengeId)
	_, err := t.Db.Table("challenge").ID(challenge.ChallengeId).Update(&challenge)
	//_, err := t.Db.Table("challenge").ID(challenge.ChallengeId).MustCols("is_practicable, is_dynamic, has_attachment").Update(&challenge)
	return err
}

func (t *ChallengeRepositoryImpl) Find(req request.ChallengeFindRequest) (challenges []response.ChallengeResponse, count int64, err error) {
	isGame := challengeValidator.IsIdValid(req.GameId) && challengeValidator.IsIdValid(req.GameId)
	applyFilter := func(q *xorm.Session) *xorm.Session {
		if challengeValidator.IsCategoryStringValid(req.Category) {
			q = q.Where("category = ?", req.Category)
		}
		if challengeValidator.IsTitleStringValid(req.Title) {
			q = q.Where("title LIKE ?", "%"+req.Title+"%")
		}
		if challengeValidator.IsPracticableIntValid(req.IsPracticable) {
			q = q.Where("is_practicable = ?", req.IsPracticable == 1)
		}
		if challengeValidator.IsDynamicIntValid(req.IsDynamic) {
			q = q.Where("is_dynamic = ?", req.IsDynamic == 1)
		}
		if challengeValidator.IsDifficultyIntValid(req.Difficulty) {
			q = q.Where("difficulty = ?", req.Difficulty)
		}
		if isGame {
			q = q.Join("INNER",
				"game_challenge",
				"game_challenge.challenge_id = challenge.id AND game_challenge.game_id = ?", req.GameId)
		}
		if challengeValidator.IsIdArrayValid(req.ChallengeIds) {
			q = q.In("challenge.id", req.ChallengeIds)
		}
		return q
	}
	db := applyFilter(t.Db.Table("challenge"))
	ct := applyFilter(t.Db.Table("challenge"))
	count, err = ct.Count(&entity.Challenge{})
	if len(req.SortBy) > 0 {
		sortKey := req.SortBy[0]
		sortOrder := req.SortBy[1]
		if sortOrder == "asc" {
			db = db.Asc("challenge." + sortKey)
		} else if sortOrder == "desc" {
			db = db.Desc("challenge." + sortKey)
		}
	} else {
		db = db.Asc("challenge.id") // 默认采用 ID 升序排列
	}
	if req.Page != 0 && req.Size > 0 {
		offset := (req.Page - 1) * req.Size
		db = db.Limit(req.Size, offset)
	}
	if isGame {
		db = db.Join(
			"LEFT",
			"submission",
			"submission.challenge_id = challenge.id AND submission.status = 2 AND submission.team_id = ?",
			req.TeamId)
	} else {
		db = db.Join(
			"LEFT",
			"submission",
			"submission.challenge_id = challenge.id AND submission.status = 2 AND submission.game_id = 0 AND submission.user_id = ?",
			req.UserId)
	}
	err = db.Find(&challenges)
	return challenges, count, err
}

func (t *ChallengeRepositoryImpl) FindById(id int64, isDetailed int) (challenge entity.Challenge, err error) {
	db := t.Db.Table("challenge").ID(id)
	if isDetailed == 0 {
		db = db.Omit("flag", "flag_fmt", "flag_env", "image")
	}
	_, err = db.Get(&challenge)
	return challenge, err
}
