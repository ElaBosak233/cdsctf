package repository

import (
	"github.com/elabosak233/cloudsdale/internal/model"
	"github.com/elabosak233/cloudsdale/internal/model/dto/request"
	"gorm.io/gorm"
)

type IChallengeRepository interface {
	Insert(challenge model.Challenge) (c model.Challenge, err error)
	Update(challenge model.Challenge) (c model.Challenge, err error)
	Delete(id uint) error
	FindById(id uint, isDetailed int) (challenge model.Challenge, err error)
	Find(req request.ChallengeFindRequest) (challenges []model.Challenge, count int64, err error)
}

type ChallengeRepository struct {
	Db *gorm.DB
}

func NewChallengeRepository(Db *gorm.DB) IChallengeRepository {
	return &ChallengeRepository{Db: Db}
}

func (t *ChallengeRepository) Insert(challenge model.Challenge) (c model.Challenge, err error) {
	result := t.Db.Table("challenges").Create(&challenge)
	return challenge, result.Error
}

func (t *ChallengeRepository) Delete(id uint) error {
	result := t.Db.Table("challenges").Delete(&model.Challenge{ID: id})
	return result.Error
}

func (t *ChallengeRepository) Update(challenge model.Challenge) (c model.Challenge, err error) {
	result := t.Db.Table("challenges").Model(&challenge).Updates(&challenge)
	return challenge, result.Error
}

func (t *ChallengeRepository) Find(req request.ChallengeFindRequest) (challenges []model.Challenge, count int64, err error) {
	isGame := req.GameID != nil && req.TeamID != nil
	applyFilter := func(q *gorm.DB) *gorm.DB {
		if req.CategoryID != nil {
			q = q.Where("category_id = ?", *(req.CategoryID))
		}
		if req.Title != "" {
			q = q.Where("title LIKE ?", "%"+req.Title+"%")
		}
		if req.IsPracticable != nil {
			q = q.Where("is_practicable = ?", *(req.IsPracticable))
		}
		if req.IsDynamic != nil {
			q = q.Where("is_dynamic = ?", *(req.IsDynamic))
		}
		if req.Difficulty > 0 {
			q = q.Where("difficulty = ?", req.Difficulty)
		}
		if isGame {
			q = q.Joins("INNER JOIN game_challenges ON game_challenges.challenge_id = challenges.id AND game_challenges.game_id = ?", *(req.GameID))
		}
		if len(req.IDs) > 0 {
			q = q.Where("(challenges.id) IN ?", req.IDs)
		}
		return q
	}
	db := applyFilter(t.Db.Table("challenges"))

	result := db.Model(&model.Challenge{}).Count(&count)
	if len(req.SortBy) > 0 {
		sortKey := req.SortBy[0]
		sortOrder := req.SortBy[1]
		if sortOrder == "asc" {
			db = db.Order("challenges." + sortKey + " ASC")
		} else if sortOrder == "desc" {
			db = db.Order("challenges." + sortKey + " DESC")
		}
	} else {
		db = db.Order("challenges.id ASC")
	}
	if req.Page != 0 && req.Size > 0 {
		offset := (req.Page - 1) * req.Size
		db = db.Offset(offset).Limit(req.Size)
	}

	result = db.
		Preload("Category").
		Preload("Flags").
		Preload("Images", func(Db *gorm.DB) *gorm.DB {
			return Db.
				Preload("Ports").
				Preload("Envs")
		}).
		Preload("Submissions", func(Db *gorm.DB) *gorm.DB {
			return Db.
				Preload("User", func(Db *gorm.DB) *gorm.DB {
					return Db.Select([]string{"id", "username", "nickname", "email"})
				}).
				Preload("Team").
				Preload("Game").
				Order("submissions.created_at ASC").
				Limit(req.SubmissionQty)
		}).
		Find(&challenges)
	return challenges, count, result.Error
}

func (t *ChallengeRepository) FindById(id uint, isDetailed int) (challenge model.Challenge, err error) {
	result := t.Db.Table("challenges").
		Where("id = ?", id).
		First(&challenge)
	return challenge, result.Error
}
