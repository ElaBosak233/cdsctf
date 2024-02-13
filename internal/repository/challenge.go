package repository

import (
	"fmt"
	"github.com/elabosak233/pgshub/internal/model"
	"github.com/elabosak233/pgshub/internal/model/dto/request"
	"github.com/elabosak233/pgshub/internal/model/dto/response"
	challengeValidator "github.com/elabosak233/pgshub/pkg/validator/challenge"
	"xorm.io/xorm"
)

type IChallengeRepository interface {
	Insert(challenge model.Challenge) (c model.Challenge, err error)
	Update(challenge model.Challenge) (c model.Challenge, err error)
	Delete(id int64) error
	FindById(id int64, isDetailed int) (challenge model.Challenge, err error)
	Find(req request.ChallengeFindRequest) (challenges []response.ChallengeResponse, count int64, err error)
}

type ChallengeRepository struct {
	Db *xorm.Engine
}

func NewChallengeRepository(Db *xorm.Engine) IChallengeRepository {
	return &ChallengeRepository{Db: Db}
}

func (t *ChallengeRepository) Insert(challenge model.Challenge) (c model.Challenge, err error) {
	_, err = t.Db.Table("challenge").Insert(&challenge)
	return challenge, err
}

func (t *ChallengeRepository) Delete(id int64) error {
	var challenge model.Challenge
	_, err := t.Db.Table("challenge").ID(id).Delete(&challenge)
	return err
}

func (t *ChallengeRepository) Update(challenge model.Challenge) (c model.Challenge, err error) {
	fmt.Println(challenge.ID)
	_, err = t.Db.Table("challenge").ID(challenge.ID).Update(&challenge)
	return challenge, err
}

func (t *ChallengeRepository) Find(req request.ChallengeFindRequest) (challenges []response.ChallengeResponse, count int64, err error) {
	isGame := challengeValidator.IsIdValid(req.GameID) && challengeValidator.IsIdValid(req.GameID)
	applyFilter := func(q *xorm.Session) *xorm.Session {
		if challengeValidator.IsCategoryStringValid(req.Category) {
			q = q.Where("category = ?", req.Category)
		}
		if challengeValidator.IsTitleStringValid(req.Title) {
			q = q.Where("title LIKE ?", "%"+req.Title+"%")
		}
		if req.IsPracticable != nil {
			q = q.Where("is_practicable = ?", *(req.IsPracticable))
		}
		if req.IsDynamic != nil {
			q = q.Where("is_dynamic = ?", *(req.IsDynamic))
		}
		if challengeValidator.IsDifficultyIntValid(req.Difficulty) {
			q = q.Where("difficulty = ?", req.Difficulty)
		}
		if isGame {
			q = q.Join("INNER",
				"game_challenge",
				"game_challenge.challenge_id = challenge.id AND game_challenge.game_id = ?", req.GameID)
		}
		if challengeValidator.IsIdArrayValid(req.IDs) {
			q = q.In("challenge.id", req.IDs)
		}
		return q
	}
	db := applyFilter(t.Db.Table("challenge"))
	ct := applyFilter(t.Db.Table("challenge"))
	count, err = ct.Count(&model.Challenge{})
	if len(req.SortBy) > 0 {
		sortKey := req.SortBy[0]
		sortOrder := req.SortBy[1]
		if sortOrder == "asc" {
			db = db.Asc("challenge." + sortKey)
		} else if sortOrder == "desc" {
			db = db.Desc("challenge." + sortKey)
		}
	} else {
		db = db.Asc("challenge.id") // 默认采用 IDs 升序排列
	}
	if req.Page != 0 && req.Size > 0 {
		offset := (req.Page - 1) * req.Size
		db = db.Limit(req.Size, offset)
	}
	//if isGame {
	//	db = db.Join(
	//		"LEFT",
	//		"submission",
	//		"submission.challenge_id = challenge.id AND submission.status = 2 AND submission.team_id = ?",
	//		req.ID)
	//} else {
	//	db = db.Join(
	//		"LEFT",
	//		"submission",
	//		"submission.challenge_id = challenge.id AND submission.status = 2 AND submission.game_id = 0 AND submission.user_id = ?",
	//		req.ID)
	//}
	err = db.Cols("challenge.*").Find(&challenges)
	return challenges, count, err
}

func (t *ChallengeRepository) FindById(id int64, isDetailed int) (challenge model.Challenge, err error) {
	db := t.Db.Table("challenge").ID(id)
	if isDetailed == 0 {
		db = db.Omit("flag", "flag_fmt", "flag_env", "image")
	}
	_, err = db.Get(&challenge)
	return challenge, err
}
